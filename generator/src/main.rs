use rand::distr::Distribution;
use rand::distr::weighted::WeightedIndex;
use std::sync::LazyLock;
use std::{collections::HashMap, fs::File, io::Write};

enum Node<I: ToString, L: ToString> {
    Inner(I),
    Leaf(L),
}
use rand::{prelude::*, rng};

// Get an RNG:

// type N = Node<InnerNode, LeafNode>;

struct InnerNode {
    tab_level: usize,
    children: Vec<TreeNode>,
}

impl ToString for InnerNode {
    fn to_string(&self) -> String {
        let mut string = "".to_string();
        for node in &self.children {
            string += &node.to_string();
        }
        string
    }
}

struct LeafNode {
    tabs: usize,
    token: String,
    new_lines: usize,
}

impl ToString for LeafNode {
    fn to_string(&self) -> String {
        // self.token.clone()
        format!(
            "{}{}{}",
            "\t".repeat(self.tabs),
            self.token,
            "\n".repeat(self.new_lines)
        )
    }
}

impl<I: ToString, L: ToString> Node<I, L> {
    fn to_string(&self) -> String {
        match self {
            Node::Inner(info) => info.to_string(),
            Node::Leaf(info) => info.to_string(),
        }
    }

    fn print(&self) {
        println!("{}", self.to_string());
    }

    fn write(&self, mut file: &File) -> std::io::Result<()> {
        file.write(self.to_string().as_str().as_bytes())?;
        Ok(())
    }
}

type TreeNode = Node<InnerNode, LeafNode>;

fn union_members(attributes: &mut Attributes) -> (Vec<TreeNode>, MemberMap) {
    let mut rng = rand::rng();
    let mut members = MemberMap::new();
    let mut curr_members = union_members_inner(0, attributes, &mut members);
    let mut curr_member_id = 1;
    while (rng.random::<u32>() % 3) != 0 {
        curr_members.append(&mut union_members_inner(
            curr_member_id,
            attributes,
            &mut members,
        ));
        curr_member_id += 1;
    }
    (curr_members, members)
}

fn union_members_inner(
    member_id: usize,
    attributes: &mut Attributes,
    members: &mut MemberMap,
) -> Vec<TreeNode> {
    let member_name = format!("member{}", member_id);
    let member_type = r#type(attributes, 0, 0);
    members.insert(member_name.clone(), member_type.token.clone());

    vec![
        Node::Leaf(LeafNode {
            tabs: 1,
            token: member_name,
            new_lines: 0,
        }),
        Node::Leaf(LeafNode {
            tabs: 0,
            token: "(".to_string(),
            new_lines: 0,
        }),
        Node::Leaf(member_type),
        Node::Leaf(LeafNode {
            tabs: 0,
            token: ")".to_string(),
            new_lines: 1,
        }),
    ]
}

fn union(attributes: &mut Attributes) -> TreeNode {
    let union_name = format!("Union{}", attributes.union_id);
    let (mut member_nodes, member_map) = union_members(attributes);
    attributes.union_map.insert(union_name.clone(), member_map);
    let mut children = vec![
        Node::Leaf(LeafNode {
            tabs: 0,
            token: "union ".to_string(),
            new_lines: 0,
        }),
        Node::Leaf(LeafNode {
            tabs: 0,
            token: union_name,
            new_lines: 1,
        }),
    ];
    children.append(&mut member_nodes);
    children.push(Node::Leaf(LeafNode {
        tabs: 0,
        token: "end".to_string(),
        new_lines: 2,
    }));
    attributes.union_id += 1;
    Node::Inner(InnerNode {
        tab_level: 0,
        children,
    })
}

fn struct_members(attributes: &mut Attributes) -> (Vec<TreeNode>, MemberMap) {
    let mut rng = rand::rng();
    let mut members = MemberMap::new();
    let mut curr_members = struct_members_inner(0, attributes, &mut members);
    let mut curr_member_id = 1;

    while (rng.random::<u32>() % 3) != 0 {
        curr_members.append(&mut struct_members_inner(
            curr_member_id,
            attributes,
            &mut members,
        ));
        curr_member_id += 1;
    }
    (curr_members, members)
}

fn struct_members_inner(
    member_id: usize,
    attributes: &mut Attributes,
    members: &mut MemberMap,
) -> Vec<TreeNode> {
    let member_name = format!("member{}", member_id);
    let member_type = type_blacklisted(attributes, vec!["Nothing".to_string()], 0, 0);
    members.insert(member_name.clone(), member_type.token.clone());
    vec![
        Node::Leaf(LeafNode {
            tabs: 1,
            token: member_name,
            new_lines: 0,
        }),
        Node::Leaf(LeafNode {
            tabs: 0,
            token: ": ".to_string(),
            new_lines: 0,
        }),
        Node::Leaf(member_type),
        Node::Leaf(LeafNode {
            tabs: 0,
            token: "".to_string(),
            new_lines: 1,
        }),
    ]
}

fn r#struct(attributes: &mut Attributes) -> TreeNode {
    let struct_name = format!("Struct{}", attributes.struct_id);
    let (mut member_nodes, member_map) = struct_members(attributes);
    attributes
        .struct_map
        .insert(struct_name.clone(), member_map);
    let mut children = vec![
        Node::Leaf(LeafNode {
            tabs: 0,
            token: "struct ".to_string(),
            new_lines: 0,
        }),
        Node::Leaf(LeafNode {
            tabs: 0,
            token: struct_name,
            new_lines: 1,
        }),
    ];
    children.append(&mut member_nodes);
    children.push(Node::Leaf(LeafNode {
        tabs: 0,
        token: "end".to_string(),
        new_lines: 2,
    }));
    attributes.struct_id += 1;
    Node::Inner(InnerNode {
        tab_level: 0,
        children,
    })
}

fn r#type(attributes: &mut Attributes, tabs: usize, new_lines: usize) -> LeafNode {
    let mut rng = rand::rng();
    let mut types = vec![
        "Int".to_string(),
        "Float".to_string(),
        "Bool".to_string(),
        "Nothing".to_string(),
    ];

    types.append(&mut attributes.get_union_types());
    types.append(&mut attributes.get_struct_types());
    LeafNode {
        tabs: tabs,
        token: types.choose(&mut rng).unwrap().clone(),
        new_lines: new_lines,
    }
}

// can fail later on!!!
fn type_whitelisted(
    attributes: &mut Attributes,
    whitelist: Vec<String>,
    tabs: usize,
    new_lines: usize,
) -> LeafNode {
    let mut rng = rand::rng();
    let mut types = vec![
        "Int".to_string(),
        "Float".to_string(),
        "Bool".to_string(),
        "Nothing".to_string(),
    ];

    types.append(&mut attributes.get_union_types());
    types.append(&mut attributes.get_struct_types());

    types = types
        .into_iter()
        .filter(|type_id| whitelist.contains(type_id))
        .collect();
    LeafNode {
        tabs: tabs,
        token: types.choose(&mut rng).unwrap().clone(),
        new_lines: new_lines,
    }
}

fn type_blacklisted(
    attributes: &mut Attributes,
    blacklist: Vec<String>,
    tabs: usize,
    new_lines: usize,
) -> LeafNode {
    let mut rng = rand::rng();
    let mut types = vec![
        "Int".to_string(),
        "Float".to_string(),
        "Bool".to_string(),
        "Nothing".to_string(),
    ];

    types.append(&mut attributes.get_union_types());
    types.append(&mut attributes.get_struct_types());

    types = types
        .into_iter()
        .filter(|type_id| !blacklist.contains(type_id))
        .collect();
    LeafNode {
        tabs: tabs,
        token: types.choose(&mut rng).unwrap().clone(),
        new_lines: new_lines,
    }
}

type MemberMap = HashMap<String, String>;

type UnionMap = HashMap<String, MemberMap>;
type StructMap = HashMap<String, MemberMap>;

struct Attributes {
    union_id: usize,
    struct_id: usize,
    function_id: usize,
    union_map: UnionMap,
    struct_map: StructMap,
    type_context: Vec<String>,
    is_start_expression: bool,
    is_end_expression: bool,
    tab_level: usize,
}

impl Attributes {
    fn get_union_types(&self) -> Vec<String> {
        self.union_map
            .clone()
            .into_iter()
            .map(|(k, _)| k.clone())
            .collect()
    }

    fn get_struct_types(&self) -> Vec<String> {
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
            type_context: Vec::new(),
            is_start_expression: false,
            is_end_expression: false,
            tab_level: 0,
        }
    }
}

fn program() -> TreeNode {
    let mut rng = rand::rng();
    let mut attributes = Attributes::default();
    let mut children: Vec<TreeNode> = vec![];

    while (rng.random::<u32>() % 3) != 0 {
        children.push(toplevel(&mut attributes));
    }
    Node::Inner(InnerNode {
        tab_level: 0,
        children,
    })
}

fn toplevel(attributes: &mut Attributes) -> TreeNode {
    let mut rng = rand::rng();
    match rng.random::<u32>() % 3 {
        0 => union(attributes),
        1 => r#struct(attributes),
        _ => function(attributes),
    }
}

fn function(attributes: &mut Attributes) -> TreeNode {
    let mut children: Vec<TreeNode> = vec![];
    let return_value = type_whitelisted(attributes, vec!["Int".to_string()], 0, 1);
    attributes.type_context.push(return_value.token.clone());
    children.push(Node::Leaf(LeafNode {
        tabs: 0,
        token: "fn ".to_string(),
        new_lines: 0,
    }));

    children.push(Node::Leaf(LeafNode {
        tabs: 0,
        token: format!("function{}", attributes.function_id),
        new_lines: 0,
    }));
    attributes.function_id += 1;
    children.push(function_type_specification(attributes));
    children.push(Node::Leaf(return_value));
    attributes.is_end_expression = true;
    attributes.is_start_expression = true;
    attributes.tab_level += 1;
    children.push(expression(attributes));
    attributes.tab_level -= 1;
    attributes.is_start_expression = false;
    attributes.is_end_expression = false;

    children.push(Node::Leaf(LeafNode {
        tabs: 0,
        token: "end".to_string(),
        new_lines: 2,
    }));
    attributes.type_context.pop();
    Node::Inner(InnerNode {
        tab_level: 0,
        children: children,
    })
}

type Acceptor = fn(&Attributes) -> bool;
type Expression = fn(&mut Attributes) -> TreeNode;

static EXPRESSIONS: LazyLock<Vec<(Acceptor, Expression, f64)>> = LazyLock::new(|| {
    vec![
        (literal_int_guard, literal_int, 1f64),
        (add_expression_guard, add_expression, 2f64),
        (sub_expression_guard, sub_expression, 2f64),
        (mul_expression_guard, mul_expression, 2f64),
    ]
});

fn expression(attributes: &mut Attributes) -> TreeNode {
    let expression = &*EXPRESSIONS
        .iter()
        .filter(|&&(guard, _, _)| guard(attributes))
        .map(|&(_, expr, weight)| (expr, weight))
        .collect::<Vec<(Expression, f64)>>();
    // expression.choose(&mut rand::rng()).unwrap()(attributes)
    choose_expression(&expression.to_vec())(attributes)
}

fn choose_expression(expressions: &Vec<(Expression, f64)>) -> Expression {
    let (expr, weights): (Vec<Expression>, Vec<f64>) = expressions.clone().into_iter().unzip();
    let weight_sum: f64 = weights.iter().sum();
    let weights_normalized: Vec<f64> = weights.iter().map(|w| (1f64 / w) / weight_sum).collect();
    let dist = WeightedIndex::new(weights_normalized).ok().unwrap();
    let mut rng = rand::rng();
    expr[dist.sample(&mut rng)]
}

fn literal_int_guard(attributes: &Attributes) -> bool {
    attributes.type_context.last().unwrap() == "Int"
}

fn literal_int(attributes: &mut Attributes) -> TreeNode {
    let mut rng = rand::rng();
    let mut value = rng.random::<i16>();
    while value == 0 {
        value = rng.random::<i16>();
    }
    let tabs = if attributes.is_start_expression {
        attributes.tab_level
    } else {
        0
    };
    let new_lines = if attributes.is_end_expression { 1 } else { 0 };
    Node::Leaf(LeafNode {
        tabs: tabs,
        token: value.to_string(),
        new_lines: new_lines,
    })
}

fn add_expression_guard(attributes: &Attributes) -> bool {
    let return_type = attributes.type_context.last().unwrap();
    return_type == "Int" || return_type == "Float"
}

fn add_expression(attributes: &mut Attributes) -> TreeNode {
    let mut children: Vec<TreeNode> = vec![];
    let is_end_expression_save = attributes.is_end_expression;
    attributes.is_end_expression = false;
    children.push(expression(attributes));
    children.push(Node::Leaf(LeafNode {
        tabs: 0,
        token: " + ".to_string(),
        new_lines: 0,
    }));
    attributes.is_start_expression = false;
    attributes.is_end_expression = is_end_expression_save;
    children.push(expression(attributes));

    Node::Inner(InnerNode {
        tab_level: attributes.tab_level,
        children: children,
    })
}

fn sub_expression_guard(attributes: &Attributes) -> bool {
    let return_type = attributes.type_context.last().unwrap();
    return_type == "Int" || return_type == "Float"
}

fn sub_expression(attributes: &mut Attributes) -> TreeNode {
    let mut children: Vec<TreeNode> = vec![];
    let is_end_expression_save = attributes.is_end_expression;
    attributes.is_end_expression = false;
    children.push(expression(attributes));
    children.push(Node::Leaf(LeafNode {
        tabs: 0,
        token: " - ".to_string(),
        new_lines: 0,
    }));
    attributes.is_start_expression = false;
    attributes.is_end_expression = is_end_expression_save;
    children.push(expression(attributes));

    Node::Inner(InnerNode {
        tab_level: attributes.tab_level,
        children: children,
    })
}

fn mul_expression_guard(attributes: &Attributes) -> bool {
    let return_type = attributes.type_context.last().unwrap();
    return_type == "Int" || return_type == "Float"
}

fn mul_expression(attributes: &mut Attributes) -> TreeNode {
    let mut children: Vec<TreeNode> = vec![];
    let is_end_expression_save = attributes.is_end_expression;
    attributes.is_end_expression = false;
    children.push(expression(attributes));
    children.push(Node::Leaf(LeafNode {
        tabs: 0,
        token: " * ".to_string(),
        new_lines: 0,
    }));
    attributes.is_start_expression = false;
    attributes.is_end_expression = is_end_expression_save;
    children.push(expression(attributes));

    Node::Inner(InnerNode {
        tab_level: attributes.tab_level,
        children: children,
    })
}

fn function_type_specification(attributes: &mut Attributes) -> TreeNode {
    let mut children: Vec<TreeNode> = vec![];
    let mut rng = rand::rng();
    children.push(Node::Leaf(LeafNode {
        tabs: 0,
        token: "(".to_string(),
        new_lines: 0,
    }));

    let mut param_id = 0usize;

    if rng.random::<u32>() % 2 != 0 {
        children.push(function_arg(attributes, param_id));
        param_id += 1;

        while rng.random::<u32>() % 3 != 0 {
            children.push(Node::Leaf(LeafNode {
                tabs: 0,
                token: ", ".to_string(),
                new_lines: 0,
            }));
            children.push(function_arg(attributes, param_id));
            param_id += 1;
        }
    }

    children.push(Node::Leaf(LeafNode {
        tabs: 0,
        token: ") ".to_string(),
        new_lines: 0,
    }));
    Node::Inner(InnerNode {
        tab_level: 0,
        children: children,
    })
}

fn function_arg(attributes: &mut Attributes, param_id: usize) -> TreeNode {
    let mut children: Vec<TreeNode> = vec![];
    children.push(Node::Leaf(LeafNode {
        tabs: 0,
        token: format!("param{}", param_id),
        new_lines: 0,
    }));

    children.push(Node::Leaf(LeafNode {
        tabs: 0,
        token: ": ".to_string(),
        new_lines: 0,
    }));
    children.push(Node::Leaf(type_blacklisted(
        attributes,
        vec!["Nothing".to_string()],
        0,
        0,
    )));
    Node::Inner(InnerNode {
        tab_level: 0,
        children: children,
    })
}

fn main() {
    // let union_string = union(&mut 0).to_string();

    // println!("{}", union_string);

    // println!("{}", r#struct(&mut 0).to_string())

    println!("{}", program().to_string());
}
