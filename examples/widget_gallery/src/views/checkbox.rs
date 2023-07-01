use vizia::prelude::*;

#[derive(Lens)]
pub struct CheckboxData {
    check1: bool,
    check2: bool,
}

pub enum CheckboxEvent {
    Toggle,
}

impl Model for CheckboxData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|checkbox_event, _| match checkbox_event {
            CheckboxEvent::Toggle => {
                self.check1 ^= true;
                self.check2 ^= true;
            }
        });
    }
}

pub fn checkbox(cx: &mut Context) {
    CheckboxData { check1: true, check2: false }.build(cx);

    VStack::new(cx, |cx| {
        Label::new(cx, "Checkbox").class("title");
        Label::new(cx, "A checkbox can be used to display a boolean value, or to select one or more items from a set of options.")
        .class("paragraph");

        Label::new(cx, "Basic checkboxes").class("header");

        HStack::new(cx, |cx|{
            Checkbox::new(cx, CheckboxData::check1).on_toggle(|cx| cx.emit(CheckboxEvent::Toggle));
            Checkbox::new(cx, CheckboxData::check2).on_toggle(|cx| cx.emit(CheckboxEvent::Toggle));
            Checkbox::new(cx, CheckboxData::check2).on_toggle(|cx| cx.emit(CheckboxEvent::Toggle)).disabled(true);
            Checkbox::new(cx, CheckboxData::check1).on_toggle(|cx| cx.emit(CheckboxEvent::Toggle)).disabled(true);

        }).class("region");

        Label::new(cx, "Checkbox and label").class("header");
        Label::new(cx, "The describing modifier can be used to link a label to a particular checkbox. Pressing on the label will then toggle the corresponding checkbox.")
        .class("paragraph");
        HStack::new(cx, |cx|{
            VStack::new(cx, |cx|{
                HStack::new(cx, |cx| {
                    Checkbox::new(cx, CheckboxData::check1)
                        .id("checky")
                        .on_toggle(|cx| cx.emit(CheckboxEvent::Toggle));
                    Label::new(cx, "Label").describing("checky");
                })
                .size(Auto)
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0))
                .col_between(Pixels(5.0));
    
                HStack::new(cx, |cx| {
                    Checkbox::new(cx, CheckboxData::check2)
                        .id("checky")
                        .on_toggle(|cx| cx.emit(CheckboxEvent::Toggle));
                    Label::new(cx, "Read-only").describing("checky");
                })
                .size(Auto)
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0))
                .col_between(Pixels(5.0));
    
                HStack::new(cx, |cx| {
                    Checkbox::new(cx, CheckboxData::check1)
                        .id("checky")
                        .on_toggle(|cx| cx.emit(CheckboxEvent::Toggle));
                    Label::new(cx, "Disabled").describing("checky");
                })
                .disabled(true)
                .size(Auto)
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0))
                .col_between(Pixels(5.0));
            }).class("group");
        }).class("region");

        HStack::new(cx, |cx|{
            Group::new(cx, "Gender", |cx|{
                HStack::new(cx, |cx| {
                    Checkbox::new(cx, CheckboxData::check1)
                        .id("c1")
                        .on_toggle(|cx| cx.emit(CheckboxEvent::Toggle));
                    Label::new(cx, "Male").describing("c1");
                })
                .size(Auto)
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0))
                .col_between(Pixels(5.0));

                HStack::new(cx, |cx| {
                    Checkbox::new(cx, CheckboxData::check1)
                        .id("c2")
                        .on_toggle(|cx| cx.emit(CheckboxEvent::Toggle));
                    Label::new(cx, "Female").describing("c2");
                })
                .size(Auto)
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0))
                .col_between(Pixels(5.0));

                HStack::new(cx, |cx| {
                    Checkbox::new(cx, CheckboxData::check1)
                        .id("c3")
                        .on_toggle(|cx| cx.emit(CheckboxEvent::Toggle));
                    Label::new(cx, "Other").describing("c3");
                })
                .size(Auto)
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0))
                .col_between(Pixels(5.0));
            });
        }).class("region");

        HStack::new(cx, |cx|{
            CheckboxInput::new(cx, |cx| cx.emit(CheckboxEvent::Toggle), CheckboxData::check1, "Test");
        }).class("region");
    })
    .class("panel");
}


pub struct CheckboxInput {

}

impl CheckboxInput {
    pub fn new<T: ToString>(cx: &mut Context, action: impl Fn(&mut EventContext) + 'static, lens: impl Lens<Target = bool>, label: impl Res<T> + Clone) -> Handle<Self> {
        let id = cx.current().to_string();
        Self{}.build(cx, |cx|{
            Checkbox::new(cx, lens)
                .id(&id)
                .on_toggle(action);
            Label::new(cx, label).describing(&id);
        })
        .layout_type(LayoutType::Row)
        .size(Auto)
        .child_top(Stretch(1.0))
        .child_bottom(Stretch(1.0))
        .col_between(Pixels(5.0))
    }
}

impl View for CheckboxInput {
    
}

pub struct Group {}

impl Group {
    pub fn new<T: ToString>(cx: &mut Context, label: impl  Res<T> + Clone, content: impl FnOnce(&mut Context)) -> Handle<Self> {
        Group{}.build(cx, |cx|{

            let has_text = !label.get_val(cx).to_string().is_empty();
            Label::new(cx, label.clone()).class("legend").display(has_text);

            (content)(cx);
        })
    }
}

impl View for Group {
    fn element(&self) -> Option<&'static str> {
        Some("group")
    }
}