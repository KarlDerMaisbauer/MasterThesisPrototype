use std::collections::HashMap;

pub type MemberMap = HashMap<String, String>;
pub type UnionMap = HashMap<String, MemberMap>;
pub type StructMap = HashMap<String, MemberMap>;
pub type ParamMap = HashMap<String, String>;
pub type VarMap = HashMap<String, String>;
pub type FunctionMap = HashMap<String, (Vec<String>, String)>;

pub struct Attributes {
    pub union_id: usize,
    pub struct_id: usize,
    pub function_id: usize,
    pub union_map: UnionMap,
    pub struct_map: StructMap,
    pub function_map: FunctionMap,
    pub type_context: Vec<String>,
    pub is_start_expression: bool,
    pub is_end_expression: bool,
    pub tab_level: usize,
    pub max_expr_depth: usize,
    pub current_params: ParamMap,
    pub current_vars: VarMap,
    pub current_var_id: usize,
    pub let_expr_allowed: bool,
    pub is_main_func: bool,
    pub main_func_generated: bool,
}

impl Attributes {
    pub fn get_union_types(&self) -> Vec<String> {
        self.union_map
            .clone()
            .into_iter()
            .map(|(k, _)| k.clone())
            .collect()
    }

    pub fn get_struct_types(&self) -> Vec<String> {
        self.struct_map
            .clone()
            .into_iter()
            .map(|(k, _)| k.clone())
            .collect()
    }
}

impl Default for Attributes {
    fn default() -> Self {
        Attributes {
            union_id: 0,
            struct_id: 0,
            function_id: 0,
            union_map: UnionMap::new(),
            struct_map: StructMap::new(),
            function_map: FunctionMap::new(),
            type_context: Vec::new(),
            is_start_expression: false,
            is_end_expression: false,
            tab_level: 0,
            max_expr_depth: 5,
            current_params: ParamMap::new(),
            current_vars: VarMap::new(),
            current_var_id: 0,
            let_expr_allowed: true,
            is_main_func: false,
            main_func_generated: false,
        }
    }
}
