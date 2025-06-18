use crate::{ast_nodes::{program::{Program, Statement}, type_def::{TypeDefNode, TypeMember}}, codegen::{llvm_utils::to_llvm_type, CodeGenerator}, visitor::accept::Accept}; // Bring the trait into scope
// (type, function_name) -> llvm_function_name
// pub function_member_llvm_names: HashMap<(String, String), String>,

// (type) -> type_parent
// pub inherits: HashMap<String, String>,

// (type) -> type_constructor_args
// pub constructor_args_types: HashMap<String, Vec<String>>,

// (type, function_name, function_index) -> function_arguments_types
// pub types_members_functions: HashMap<(String,String,i32), Vec<String>>,

// (type, member) -> member_type
// pub type_members_types: HashMap<(String, String), String>,

// (type, member) -> member_index_on_type_struct
// pub type_members_ids: HashMap<(String, String), i32>,

impl CodeGenerator {
    pub fn init_all_type_methods_and_props(&mut self, node: &mut Program) {
        for statement in &mut node.statements {
            match statement {
                Statement::StatementTypeDef(type_def) => {
                    let type_name = type_def.identifier.clone();
                    let mut params_types_list = Vec::new();
                    for param in type_def.params.iter() {
                        let param_type = param.signature.clone();
                        params_types_list.push(param_type);
                    }
                    self.context.constructor_args_types.insert(type_name.clone(), params_types_list);
                    if let Some(parent_type) = &type_def.parent {
                        self.context.inherits.insert(type_name.clone(), parent_type.clone());
                    }
                    let mut member_index: i32 = 2;
                    for member in type_def.members.iter() {
                        match member { 
                            TypeMember::Property(assignment) => {
                                let member_name = assignment.identifier.clone();
                                self.context.type_members_ids.insert((type_name.clone(), member_name.clone()), member_index);
                                self.context.type_members_types.insert((type_name.clone(), member_name), assignment.node_type.clone().unwrap().type_name);
                                member_index += 1;
                            }
                            TypeMember::Method(method) => {
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
                }
                _ => continue
            }
        }
    }

    pub fn generate_type_table(&mut self, node: &mut TypeDefNode) {
        let type_name = node.identifier.clone();
        let mut methods_list = Vec::new();
        let mut methods_index = 0;
        for member in node.members.iter() {
            match member {
                TypeMember::Method(method) => {
                    if let Some (llvm_name) = self.context.function_member_llvm_names.get_mut(&(type_name.clone(), method.name.clone())) {
                        methods_list.push(llvm_name.clone());
                        self.context.type_functions_ids.insert((type_name.clone(), method.name.clone()), methods_index);
                        methods_index += 1;
                    }
                }
                _ => continue 
            }
        }
        //vtable only contains original methods 
        // if let Some(parent) = node.parent.clone() {
        //     for (ty, func_name) in self.context.function_member_llvm_names.keys().filter(|(ty, _)| ty == &parent) {
        //         let method_llvm = self.context.function_member_llvm_names.get(&(ty.clone(), func_name.clone())).unwrap();
        //         if !methods_list.contains(&method_llvm) {
        //             methods_list.push(method_llvm.clone());
        //             self.context.type_members_ids.insert((type_name.clone(), func_name.clone()), methods_index);
        //             methods_index += 1;
        //         }
        //     }
        // }
        let table = format!("%{}_vtable", type_name);
        let ptr_types = std::iter::repeat("ptr").take(methods_index as usize).collect::<Vec<_>>().join(", ");
        self.context.add_line(format!("{} = type {{ {} }}" , table, ptr_types)); 
        
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

        let mut methods_list = Vec::new();
        for member in node.members.iter() {
            match member {
                TypeMember::Method(method) => {
                    if let Some (llvm_name) = self.context.function_member_llvm_names.get_mut(&(type_name.clone(), method.name.clone())) {
                        methods_list.push(llvm_name.clone());
                    }
                }
                _ => continue 
            }
        }

        // vtable only contais original methods
        // if let Some(parent) = node.parent.clone() {
        //     for (ty, func_name) in self.context.function_member_llvm_names.keys().filter(|(ty, _)| ty == &parent) {
        //         let method_llvm = self.context.function_member_llvm_names.get(&(ty.clone(), func_name.clone())).unwrap();
        //         if !methods_list.contains(&method_llvm) {
        //             methods_list.push(method_llvm.clone());
        //         }
        //     }
        // }

        let table = format!("%{}_vtable", type_name);
        
        let table_id = self.context.new_id();
        let type_table_instance = format!("@{}_vtable{}", type_name,table_id);

        let method_ptrs = methods_list
            .iter()
            .map(|llvm_name| format!("ptr {}", llvm_name))
            .collect::<Vec<_>>()
            .join(", ");
        self.context.add_line(format!("{} = global {} {{ {} }}", type_table_instance, table, method_ptrs)); //Esto va en el constructor 
        
        self.context.add_line(format!("define ptr @{}_new( {} ) {{",type_name.clone(),params_str.clone())); 

        let size_temp = self.context.new_temp("Number".to_string());
        self.context.add_line(format!("{} = ptrtoint ptr getelementptr({}, ptr null, i32 1) to i64", size_temp, type_reg));
        let mem_temp = self.context.new_temp(type_name.clone());
        self.context.add_line(format!("{} = call ptr @malloc(i64 {})" , mem_temp , size_temp));

        self.context.add_line(format!("%vtable_ptr = getelementptr {}, ptr {}, i32 0, i32 0", type_reg, mem_temp));
        self.context.add_line(format!("store ptr {}, ptr %vtable_ptr", type_table_instance));

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
        }
        
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

        // for param in node.params.iter() {
        //     let prop_reg = self.context.new_temp(param.signature.clone());

        //     let member_key = (type_name.clone(), param.name.clone());
        //     let member_index = self.context.type_members_ids.get(&member_key)
        //         .expect("Member index not found for type and param name");
        //     self.context.add_line(format!(
        //         "{} = getelementptr {}, ptr {}, i32 0, i32 {}",
        //         prop_reg, type_reg, mem_temp, member_index
        //     ));
        //     self.context.add_line(format!("store {} %registroconresult, ptr {}" ,to_llvm_type(param.signature.clone()), prop_reg)); //Todo
        // }
        self.context.add_line(format!("ret ptr {}", mem_temp));
        self.context.add_line("}".to_string());

    }
}

