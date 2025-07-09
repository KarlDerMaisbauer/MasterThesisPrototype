use crate::grammar::attributes::Attributes;
use crate::grammar::nodes::TerminalInfo;
use crate::grammar::utils::gen_type::gen_type_whitelisted::*;

pub fn type_whitelisted(
    attributes: &mut Attributes,
    whitelist: Vec<String>,
    tabs: usize,
    new_lines: usize,
) -> TerminalInfo {
    let data_type = gen_type_whitelisted(attributes, whitelist);
    TerminalInfo {
        tabs: tabs,
        token: data_type,
        new_lines: new_lines,
    }
}
