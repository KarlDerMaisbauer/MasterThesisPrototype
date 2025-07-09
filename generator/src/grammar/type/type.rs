use crate::grammar::attributes::Attributes;
use crate::grammar::nodes::TerminalInfo;
use crate::grammar::utils::gen_type::gen_type::*;

pub fn r#type(attributes: &mut Attributes, tabs: usize, new_lines: usize) -> TerminalInfo {
    let data_type = gen_type(attributes);
    TerminalInfo {
        tabs: tabs,
        token: data_type,
        new_lines: new_lines,
    }
}
