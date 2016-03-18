use dom::processinginstruction::ProcessingInstruction;
use dom::bindings::reflector::Reflector;
use dom::bindings::codegen::Bindings::StyleSheetBinding;
use dom::bindings::codegen::Bindings::StyleSheetBinding::StyleSheetMethods;
use util::str::DOMString;

#[dom_struct]
pub struct StyleSheet {
    reflector_: Reflector,

    type_: DOMString,
    /*href: DOMString,
    ownerNode: ProcessingInstruction,
    parentStyleSheet: StyleSheet,
    title: DOMString,
    media: MediaList,
    disabled: Cell<bool>,*/
}

impl StyleSheetMethods for StyleSheet {

    fn Type_(&self) -> DOMString {
        self.type_.clone()
    }

    // Similarly create getters for the other attributes
}
