use std::fs;

use chromiumoxide::{cdp, Page};

pub async fn inject(page: &Page) -> Result<(), Box<dyn std::error::Error>> {
    page.execute(
        cdp::browser_protocol::page::AddScriptToEvaluateOnNewDocumentParams {
            source: fs::read_to_string("src/evasions/utils.js").unwrap(),
            include_command_line_api: None,
            world_name: None,
        },
    )
    .await?;
    page.execute(
        cdp::browser_protocol::page::AddScriptToEvaluateOnNewDocumentParams {
            source: fs::read_to_string("src/evasions/chrome_app.js").unwrap(),
            include_command_line_api: None,
            world_name: None,
        },
    )
    .await?;
    page.execute(
        cdp::browser_protocol::page::AddScriptToEvaluateOnNewDocumentParams {
            source: fs::read_to_string("src/evasions/chrome_runtime.js").unwrap(),
            include_command_line_api: None,
            world_name: None,
        },
    )
    .await?;
    page.execute(
        cdp::browser_protocol::page::AddScriptToEvaluateOnNewDocumentParams {
            source: fs::read_to_string("src/evasions/iframe_content_window.js").unwrap(),
            include_command_line_api: None,
            world_name: None,
        },
    )
    .await?;
    page.execute(
        cdp::browser_protocol::page::AddScriptToEvaluateOnNewDocumentParams {
            source: fs::read_to_string("src/evasions/media_codecs.js").unwrap(),
            include_command_line_api: None,
            world_name: None,
        },
    )
    .await?;
    page.execute(
        cdp::browser_protocol::page::AddScriptToEvaluateOnNewDocumentParams {
            source: fs::read_to_string("src/evasions/navigator_language.js").unwrap(),
            include_command_line_api: None,
            world_name: None,
        },
    )
    .await?;
    page.execute(
        cdp::browser_protocol::page::AddScriptToEvaluateOnNewDocumentParams {
            source: fs::read_to_string("src/evasions/navigator_permissions.js").unwrap(),
            include_command_line_api: None,
            world_name: None,
        },
    )
    .await?;
    page.execute(
        cdp::browser_protocol::page::AddScriptToEvaluateOnNewDocumentParams {
            source: fs::read_to_string("src/evasions/navigator_plugins.js").unwrap(),
            include_command_line_api: None,
            world_name: None,
        },
    )
    .await?;
    page.execute(
        cdp::browser_protocol::page::AddScriptToEvaluateOnNewDocumentParams {
            source: fs::read_to_string("src/evasions/navigator_vendor.js").unwrap(),
            include_command_line_api: None,
            world_name: None,
        },
    )
    .await?;
    page.execute(
        cdp::browser_protocol::page::AddScriptToEvaluateOnNewDocumentParams {
            source: fs::read_to_string("src/evasions/navigator_webdriver.js").unwrap(),
            include_command_line_api: None,
            world_name: None,
        },
    )
    .await?;
    let ua = page
        .execute(cdp::browser_protocol::browser::GetVersionParams {})
        .await?;

    let mut modified_ua = String::new();
    if ua.user_agent.contains("Headless") {
        modified_ua = ua.user_agent.replace("Headless", "");
    }
    page.execute(cdp::browser_protocol::network::SetUserAgentOverrideParams {
        user_agent: modified_ua,
        accept_language: None,
        platform: None,
        user_agent_metadata: None,
    })
    .await?;

    page.execute(
        cdp::browser_protocol::page::AddScriptToEvaluateOnNewDocumentParams {
            source: fs::read_to_string("src/evasions/webgl_vendor_override.js").unwrap(),
            include_command_line_api: None,
            world_name: None,
        },
    )
    .await?;
    page.execute(
        cdp::browser_protocol::page::AddScriptToEvaluateOnNewDocumentParams {
            source: fs::read_to_string("src/evasions/window_outerdimensions.js").unwrap(),
            include_command_line_api: None,
            world_name: None,
        },
    )
    .await?;
    page.execute(
        cdp::browser_protocol::page::AddScriptToEvaluateOnNewDocumentParams {
            source: fs::read_to_string("src/evasions/hairline_fix.js").unwrap(),
            include_command_line_api: None,
            world_name: None,
        },
    )
    .await?;

    Ok(())
}
