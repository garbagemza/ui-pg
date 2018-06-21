
// The data we will visit

pub enum UIModel {
    Component(UIComponent),
    Composite(Box<UIModel>),
}

pub struct UIComponent {
    pub value: String,
}
