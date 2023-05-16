use crate::{components::*, utils::mount_style::mount_style};
use leptos::*;
use stylers::style_sheet_str;

#[component]
pub fn Card(
    cx: Scope,
    #[prop(optional)] title: MaybeSignal<String>,
    #[prop(default = None)] header: Option<Children>,
    #[prop(default = None)] header_extra: Option<Children>,
    children: Children,
    #[prop(default = None)] footer: Option<Children>,
) -> impl IntoView {
    let class_name = mount_style("card", || style_sheet_str!("./src/card/card.css"));
    view! {
        cx, class=class_name,
        <div class="melt-card">
            {
                if header.is_some() || title.get().is_empty() {
                    view! {
                        cx, class=class_name,
                        <div class="melt-card__header">
                            <div class="melt-card__header-content">
                                <OptionComp value=header view=|cx, header| {
                                    header(cx).into_view(cx)
                                }>
                                    { title.get() }
                                </OptionComp>
                            </div>
                            <OptionComp value=header_extra view=move |cx, header_extra| {
                                view! {
                                    cx, class=class_name,
                                    <div class="melt-card__header-extra">
                                        { header_extra(cx)}
                                    </div>
                                }
                            }/>
                        </div>
                    }.into()
                } else {
                    None
                }
            }
            <div class="melt-card__content">
                { children(cx) }
            </div>
            <OptionComp value=footer view=move |cx, footer| {
                view! {
                    cx, class=class_name,
                    <div class="melt-card__footer">
                        { footer(cx) }
                    </div>
                }
            }/>
         </div>
    }
}
