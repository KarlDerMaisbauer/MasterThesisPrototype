use crate::grammar::attributes::Attributes;
use crate::grammar::nodes::*;
use crate::grammar::utils::gen_type::gen_type_blacklisted::*;

pub fn type_blacklisted(
    attributes: &mut Attributes,
    blacklist: Vec<String>,
    tabs: usize,
    new_lines: usize,
) -> TerminalInfo {
    let data_type = gen_type_blacklisted(attributes, blacklist);
    TerminalInfo {
        tabs: tabs,
        token: data_type,
        new_lines: new_lines,
    }
}
