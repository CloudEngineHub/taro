use crate::{ transform_harmony::EtsDirection, utils::{ self, harmony::components } };

pub fn get_component_attr_str(node_name: &str, tag_name: &str) -> String {
    if tag_name == "text" {
        format!(
            ".attributeModifier(commonStyleModify.setNode({} as TaroElement))\n.textSpecialFontStyle(getFontAttributes({} as TaroElement))",
            node_name,
            node_name
        )
    } else if tag_name == "row" {
        format!(".attributeModifier(rowModify.setNode({} as TaroElement))", node_name)
    } else if tag_name == "column" {
        format!(".attributeModifier(columnModify.setNode({} as TaroElement))", node_name)
    } else {
        format!(".attributeModifier(commonStyleModify.setNode({} as TaroElement))", node_name)
    }
}

pub fn get_component_style_str(node_name: &str, tag_name: &str) -> String {
    format!(
        r#"{}
.onVisibleAreaChange(getNodeThresholds({node_id} as TaroElement) || [0.0, 1.0], getComponentEventCallback({node_id} as TaroElement, VISIBLE_CHANGE_EVENT_NAME))
.onAreaChange(getComponentEventCallback({node_id} as TaroElement, AREA_CHANGE_EVENT_NAME, (res: TaroAny) => {{
  ({node_id} as TaroElement)._nodeInfo.areaInfo = res[1]
}}))"#,
        get_component_attr_str(node_name, tag_name),
        node_id = node_name
    )
}

pub fn get_view_component_str(node_name: &str, child_content: &str, direction: EtsDirection) -> String {
    let component_name;
    let mut component_param = "".to_string();
    let component_children = match child_content {
        "" => "".to_string(),
        _ => format!("\n{}", child_content),
    };

    match direction {
        EtsDirection::Row => {
            component_name = "Row";
        }
        EtsDirection::Column => {
            component_name = "Column";
        }
        EtsDirection::Flex => {
            component_name = "Flex";
            component_param = format!("FlexManager.flexOptions({})", node_name);
        }
    }
    let style = get_component_style_str(node_name, component_name.to_lowercase().as_str());

    format!(
        "{name}({param}) {{{children}}}\n{style}",
        name = component_name,
        param = component_param,
        children = component_children,
        style = style
    )
}

pub fn get_image_component_str(node_name: &str) -> String {
    format!(
        "Image(({node_id} as TaroElement).getAttribute('src'))\n.objectFit(getImageMode(({node_id} as TaroElement).getAttribute('mode')))\n{style}",
        node_id = node_name,
        style = get_component_style_str(node_name, "image")
    )
}

pub fn get_text_component_str(node_name: &str) -> String {
    format!("createText({node_id} as TaroTextElement)", node_id = node_name)
}

pub fn create_component_event(event_name: &str, node_name: &str) -> String {
    let process_event_trigger_name = |name: &str| -> String {
        if name == "touch" { String::from("TOUCH_EVENT_MAP.get(e.type)") } else { format!("'{}'", name) }
    };

    format!(
        "\n.{}(e => eventHandler(e, {}, {} as TaroElement))",
        event_name,
        process_event_trigger_name(
            &event_name
                .get(2..)
                .unwrap()
                .to_lowercase()
        ),
        node_name
    )
}
