use crate::utils::mount_style::mount_style;
use leptos::*;
use stylers::style_sheet_str;

#[component]
pub fn LayoutSider(
    #[prop(optional, into)] style: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    let class_name = mount_style("layout-sider", || {
        style_sheet_str!("./src/layout/layout-sider.css")
    });
    view! {class=class_name,
        <div class="melt-layout-sider" style=move || style.get()>
            { children() }
        </div>
    }
}
