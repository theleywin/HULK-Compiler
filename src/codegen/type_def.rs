use std::{collections::HashMap};

use crate::{ast_nodes::{program::{Program, Statement}, type_def::{TypeDefNode, TypeMember}}, codegen::{llvm_utils::to_llvm_type, CodeGenerator}, visitor::accept::Accept};

impl CodeGenerator {
    pub fn init_all_type_methods_and_props(&mut self, node: &mut Program) {
        let mut type_def_list = Vec::new();
        for statement in &mut node.statements {
            match statement {
                Statement::StatementTypeDef(type_def) => {
                   type_def_list.push(type_def);
                }
                _ => continue
            }
        }
        let mut visited:  HashMap<String, bool> = HashMap::new(); 
        let node_list: Vec<&Box<TypeDefNode>> = type_def_list.iter().map(|r| &**r).collect();
        for node in &type_def_list {
            if !visited.contains_key(&node.identifier) {
                self.aux_init_types(node, node_list.clone(), &mut visited);
            }
        }
    }

    fn aux_init_types(&mut self, node: &Box<TypeDefNode>, node_list : Vec<&Box<TypeDefNode>>, visited: &mut HashMap<String, bool>) {
        if let Some(parent_name) = node.parent.clone() {
            if let Some(parent_node) = node_list.iter().find(|n| n.identifier == parent_name) {
                if !visited.contains_key(&parent_name) {
                    self.aux_init_types(parent_node, node_list.clone(), visited);
                }
            }
        }
        self.generate_type_table(&mut node.clone());
        visited.insert(node.identifier.clone(), true);
    }

    pub fn generate_type_table(&mut self, node: &mut TypeDefNode) {
        let type_name = node.identifier.clone();
        let mut methods: Vec<(String, String)> = Vec::new();
        let mut count_functions = 0;

        self.context.type_id.insert(node.identifier.clone(), self.context.count_types.clone());
        self.context.types_vtables.push(format!("@{}_vtable", node.identifier.clone()));
        self.context.count_types += 1;
    
        let mut params_types_list = Vec::new();
        for param in node.params.iter() {
            let param_type = param.signature.clone();
            params_types_list.push(param_type);
        }
        self.context.constructor_args_types.insert(type_name.clone(), params_types_list);
        if let Some(parent_type) = &node.parent {
            self.context.inherits.insert(type_name.clone(), parent_type.clone());
        }

        let mut props_list = Vec::new();
        let mut member_index: i32 = 2 ;

        if let Some(parent_name) = &node.parent {
            if let Some(parent_members) = self.context.types_members.get(parent_name) {
                for (index,( member_name,member_type)) in parent_members.iter().enumerate() {
                    self.context.type_members_types.insert((type_name.clone(), member_name.clone()), member_type.clone());
                    self.context.type_members_ids.insert((type_name.clone(), member_name.clone()), index.clone() as i32);
                    member_index += 1;
                }
                props_list = parent_members.clone();
            }
        }

        for member in node.members.iter() {
            match member { 
                TypeMember::Property(assignment) => {
                    let member_name = assignment.identifier.clone();
                    self.context.type_members_ids.insert((type_name.clone(), member_name.clone()), member_index);
                    self.context.type_members_types.insert((type_name.clone(), member_name.clone()), assignment.node_type.clone().unwrap().type_name);
                    props_list.push((member_name.clone(), assignment.node_type.clone().unwrap().type_name));
                    member_index += 1;
                }
                TypeMember::Method(method) => {
                    count_functions += 1;
                    let method_name = method.name.clone();
                    self.context.function_member_llvm_names.insert((type_name.clone(), method_name.clone()), format!("@{}_{}", type_name, method_name.clone()));
                    let mut method_args_types = Vec::new(); 
                    for param in &method.params {
                        method_args_types.push(param.signature.clone());
                    }
                    self.context.types_members_functions.insert((type_name.clone(), method_name.clone(), member_index), method_args_types);
                    self.context.type_members_types.insert((type_name.clone(),method_name.clone()), method.node_type.clone().unwrap().type_name);
                }
            }
        }
        self.context.types_members.insert(type_name.clone(), props_list);
        self.context.max_functions += count_functions;

        let mut props_types = Vec::new();
        if let Some(props_list) = self.context.types_members.get(&type_name) {
            for (_prop_name, prop_type) in props_list {
                props_types.push(to_llvm_type(prop_type.clone()));
            }
        } 

        let list_props_str = props_types
            .iter()
            .map(|llvm_name| format!("{}", llvm_name))
            .collect::<Vec<_>>()
            .join(", ");
        // type (vtable , parent , props...)
        if props_types.len() > 0 {
            self.context.add_line(format!("%{}_type = type {{ i32, ptr, {} }}", type_name.clone(), list_props_str)); 
        } else {
            self.context.add_line(format!("%{}_type = type {{ i32, ptr }}", type_name.clone())); 
        }
        
        // lll
        if let Some(parent_name) = &node.parent {
            if let Some(parent_methods) = self.context.types_functions.get(parent_name) {
                methods = parent_methods.clone();
            }
        }
        
        for member in node.members.iter() {
            match member {
                TypeMember::Method(method) => {
                    if let Some(llvm_name) = self.context.function_member_llvm_names.get(&(type_name.clone(), method.name.clone())) {
                        if let Some(idx) = methods.iter().position(|(name, _)| name == &method.name.clone()) {
                            methods[idx] = (method.name.clone(),llvm_name.clone());
                        } else {
                            methods.push((method.name.clone(), llvm_name.clone()));
                        }
                    }
                }
                _ => continue 
            }
        }
        for (index, (name, _)) in methods.iter().enumerate() {
            self.context.type_functions_ids.insert((type_name.clone(),name.clone()),index as i32);
        }
        self.context.types_functions.insert(type_name.clone(), methods);
    }

    pub fn generate_type_constructor(&mut self, node: &mut TypeDefNode){
        let type_name = node.identifier.clone();
        let type_reg = format!("%{}_type",type_name);
        let mut params_list = Vec::new();
        self.context.enter_scope();
        for param in node.params.iter() {
            let param_name  = format!("%{}.{}",param.name.clone(),self.context.get_scope());
            params_list.push(format!("ptr {}",param_name.clone()));
            self.context.add_variable(param_name.clone(), to_llvm_type(param.signature.clone()));
        }
        let params_str = params_list.join(", ");

    
        // Crea una lista del tamaño de max_functions, inicializada con "ptr null"
        let mut method_list = vec!["ptr null".to_string(); self.context.max_functions as usize];

        // Llena la lista con el nombre de la función en el índice correspondiente
        if let Some(functions) = self.context.types_functions.get(&type_name) {
            for (index,(_, llvm_name))in functions.iter().enumerate() {
                if index < self.context.max_functions as usize {
                    method_list[index] = format!("ptr {}", llvm_name);
                }
            }
        }

        // generate vtable instance 
        let type_table_instance = format!("@{}_vtable", type_name);

        // Crea la instancia de la vtable usando method_list
        self.context.add_line(format!("{} = constant %VTableType [ {} ]", type_table_instance, method_list.join(", ")));
        
        // build constructor
        self.context.add_line(format!("define ptr @{}_new( {} ) {{",type_name.clone(),params_str.clone())); 

        let size_temp = self.context.new_temp("Number".to_string());
        self.context.add_line(format!("{} = ptrtoint ptr getelementptr({}, ptr null, i32 1) to i64", size_temp, type_reg));
        let mem_temp = self.context.new_temp(type_name.clone());
        self.context.add_line(format!("{} = call ptr @malloc(i64 {})" , mem_temp , size_temp));

        // set type index on super_vtable
        self.context.add_line(format!("%index_ptr = getelementptr {}, ptr {}, i32 0, i32 0", type_reg, mem_temp));
        self.context.add_line(format!("store i32 {}, ptr %index_ptr", self.context.type_id.get(&type_name).expect("Type ID not found for type").clone()));

        if let Some(parent_name) = node.parent.clone() {
            let mut parent_args_values = Vec::new();
            for arg in node.parent_args.iter_mut() {
                let arg_result = arg.accept(self);
                let arg_reg = self.context.new_temp(arg_result.ast_type);
                self.context.add_line(format!("{} = alloca {}", arg_reg.clone() , arg_result.llvm_type.clone()));
                self.context.add_line(format!(
                    "store {} {}, ptr {}",
                    arg_result.llvm_type, arg_result.register, arg_reg.clone()
                ));
                parent_args_values.push(format!("ptr {}",arg_reg.clone()));
            }
            let args_regs_str = parent_args_values.join(", ");
            let parent_ptr = self.context.new_temp(parent_name.clone());
            let parent_constructor_name = format!("@{}_new" , parent_name.clone()); 
            self.context.add_line(format!(
                "{} = call ptr {}({})",
                parent_ptr.clone(), parent_constructor_name, args_regs_str
            ));
            self.context.add_line(format!("%parent_ptr = getelementptr {}, ptr {}, i32 0, i32 1", type_reg, mem_temp));
            self.context.add_line(format!("store ptr {}, ptr %parent_ptr", parent_ptr.clone()));
            if let Some(parent_members) = self.context.types_members.get(&parent_name) {
                let parent_members_cloned = parent_members.clone();
                for (index, (_member_name, member_type)) in parent_members_cloned.iter().enumerate() {
                    let llvm_type = to_llvm_type(member_type.clone());
                    let parent_type = format!("%{}_type", parent_name.clone());
                    self.context.add_line(format!(
                        "%src_{} = getelementptr {}, ptr {}, i32 0, i32 {}",
                        index.clone(), parent_type.clone(), parent_ptr.clone(), index.clone() + 2
                    ));
                    self.context.add_line(format!(
                        "%val_{} = load {}, ptr %src_{}",
                        index.clone(), llvm_type, index.clone()
                    ));
                    self.context.add_line(format!(
                        "%dst_{} = getelementptr {}, ptr {}, i32 0, i32 {}",
                        index.clone(), type_reg, mem_temp, index.clone() + 2
                    ));
                    self.context.add_line(format!(
                        "store {} %val_{}, ptr %dst_{}",
                        llvm_type, index.clone(), index.clone()
                    ));
                }
            }
        }
        
        // set properties values 

        for member in node.members.iter() {
            match member {
                TypeMember::Property(assign) => {
                    let prop_reg = assign.expression.clone().accept(self);
                    let result_reg = self.context.new_temp(prop_reg.ast_type.clone());
                    let member_key = (type_name.clone(), assign.identifier.clone());
                    let member_index = self.context.type_members_ids.get(&member_key)
                        .expect("Member index not found for type and param name");
                    self.context.add_line(format!(
                        "{} = getelementptr {}, ptr {}, i32 0, i32 {}",
                        result_reg, type_reg, mem_temp, member_index
                    ));
                    self.context.add_line(format!("store {} {}, ptr {}", prop_reg.llvm_type, prop_reg.register, result_reg));
                }
                _ => continue 
            }
        }

        self.context.add_line(format!("ret ptr {}", mem_temp));
        self.context.add_line("}".to_string());

    }

    pub fn generate_get_vtable_method(&mut self) {
        self.context.add_line("define ptr @get_vtable_method(i32 %type_id, i32 %method_id) {".to_string());
        self.context.add_line(format!("%vtable_ptr_ptr = getelementptr [ {} x ptr ], ptr @super_vtable, i32 0, i32 %type_id", self.context.count_types));
        self.context.add_line(format!("%vtable_ptr = load ptr , ptr %vtable_ptr_ptr"));
        self.context.add_line(format!("%typed_vtable = bitcast ptr %vtable_ptr to ptr"));
        self.context.add_line(format!("%method_ptr = getelementptr [ {} x ptr ], ptr %typed_vtable, i32 0, i32 %method_id", self.context.max_functions));
        self.context.add_line(format!("%method = load ptr, ptr %method_ptr"));
        self.context.add_line(format!("ret ptr %method"));
        self.context.add_line("}".to_string());
    }
}

