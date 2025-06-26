mod discord;
mod slack;
mod mcp_handler;
mod tools;
use crate::tools::Human;

use clap::Parser;
use discord::HumanInDiscord;
use rust_mcp_sdk::error::{McpSdkError, SdkResult};
use rust_mcp_sdk::schema::{
    Implementation, InitializeResult, ServerCapabilities, ServerCapabilitiesTools,
    LATEST_PROTOCOL_VERSION,
};

use rust_mcp_sdk::{
    mcp_server::{server_runtime, ServerRuntime},
    McpServer, StdioTransport, TransportOptions,
};
use serenity::all::{ChannelId, UserId};

#[derive(Debug, Parser)]
struct Args {
    // Discord configuration (make optional)
    #[clap(long, env = "DISCORD_TOKEN", help = "Discord bot token")]
    discord_token: Option<String>,
    #[clap(long, env = "DISCORD_CHANNEL_ID", help = "Discord channel ID")]
    discord_channel_id: Option<ChannelId>,
    #[clap(long, env = "DISCORD_USER_ID", help = "Discord user ID")]
    discord_user_id: Option<UserId>,

    // Slack configuration (new, optional)
    #[clap(long, env = "SLACK_APP_TOKEN", help = "Slack app-level token for Socket Mode")]
    slack_app_token: Option<String>,
    #[clap(long, env = "SLACK_BOT_TOKEN", help = "Slack bot token")]
    slack_bot_token: Option<String>,
    #[clap(long, env = "SLACK_CHANNEL_ID", help = "Slack channel ID")]
    slack_channel_id: Option<String>,
    #[clap(long, env = "SLACK_USER_ID", help = "Slack user ID")]
    slack_user_id: Option<String>,

    // Platform selection
    #[clap(
        long,
        env = "PLATFORM",
        default_value = "discord",
        help = "Platform to use: discord or slack"
    )]
    platform: Platform,
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum Platform {
    Discord,
    Slack,
}

#[tokio::main]
async fn main() -> SdkResult<()> {
    let args = Args::parse();

    // Basic validation
    match args.platform {
        Platform::Discord => {
            if args.discord_token.is_none() {
                eprintln!("Error: Discord token required when using Discord platform");
                eprintln!("Set DISCORD_TOKEN environment variable or use --discord-token");
                std::process::exit(1);
            }
        }
        Platform::Slack => {
            if args.slack_app_token.is_none() || args.slack_bot_token.is_none() {
                eprintln!("Error: Slack tokens required when using Slack platform");
                eprintln!("Set SLACK_APP_TOKEN and SLACK_BOT_TOKEN environment variables");
                std::process::exit(1);
            }
        }
    }

    println!("Starting Human-in-the-Loop MCP server with {} platform",
             match args.platform {
                 Platform::Discord => "Discord",
                 Platform::Slack => "Slack",
             });

    // For now, continue with existing Discord implementation
    // This will be updated in later tickets
    match args.platform {
        Platform::Discord => {
            // Existing Discord code...
            let Args {
                discord_token: Some(discord_token),
                discord_channel_id: Some(discord_channel_id),
                discord_user_id: Some(discord_user_id),
                ..
            } = args else {
                eprintln!("Missing required Discord configuration");
                std::process::exit(1);
            };

            // Continue with existing Discord implementation
            let human = HumanInDiscord::new(discord_user_id, discord_channel_id);
            let discord = discord::start(&discord_token, human.handler().clone());

            let server_details = InitializeResult {
                server_info: Implementation {
                    name: "Human in the loop".to_string(),
                    version: "0.1.0".to_string(),
                },
                capabilities: ServerCapabilities {
                    tools: Some(ServerCapabilitiesTools { list_changed: None }),
                    ..Default::default()
                },
                meta: None,
                instructions: Some(format!(
                    "This is a Human-in-the-Loop MCP server using {} platform. \
                     Use the 'ask_human' tool when you need information from humans.",
                    match args.platform {
                        Platform::Discord => "Discord",
                        Platform::Slack => "Slack",
                    }
                )),
                protocol_version: LATEST_PROTOCOL_VERSION.to_string(),
            };

            let transport = StdioTransport::new(TransportOptions::default())?;
            let server: ServerRuntime = server_runtime::create_server(
                server_details,
                transport,
                mcp_handler::Handler::new(human)
            );
            let mcp = server.start();

            tokio::select! {
                res = mcp => res?,
                res = discord => res.map_err(|e| McpSdkError::AnyError(e.into_boxed_dyn_error()))?,
            }
        }
        Platform::Slack => {
            println!("Slack platform selected - using placeholder implementation");
            // For now, just create a placeholder and exit cleanly
            let slack = crate::slack::HumanInSlack::new(
                args.slack_user_id.unwrap_or_default(),
                args.slack_channel_id.unwrap_or_default(),
            );

            let test_response = slack.ask("Hello from Slack!").await
                .map_err(|e| McpSdkError::AnyError(e.into_boxed_dyn_error()))?;
            println!("Slack placeholder response: {}", test_response);

            // TODO: Full Slack implementation will be added in later tickets
            println!("Slack integration coming soon! For now, use Discord platform.");
        }
    }

    Ok(())
}
