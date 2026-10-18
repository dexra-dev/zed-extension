# Dexra MCP Server for Zed

Adds [Dexra](https://dexra.dev) to Zed's agent: Solana DeFi checks for AI agents. Before your agent buys, sells or provides liquidity, it can check a token launch, a token's real holders, a wallet, a pool, a perp market or a planned swap. Each check returns a verdict (`ok`, `caution`, `avoid` or `unknown`) with the reasons and the evidence.

The extension installs the [`@dexra-dev/mcp`](https://www.npmjs.com/package/@dexra-dev/mcp) bridge from npm with Zed's own Node.js and connects it to `https://api.dexra.dev/mcp`.

## Setup

1. Install **Dexra MCP Server** from Zed's extensions page.
2. Optional: add an API key in your Zed settings:

```json
{
  "context_servers": {
    "dexra-mcp-server": {
      "settings": {
        "api_key": "dxr_..."
      }
    }
  }
}
```

Without a key the free tools (`describe_address`, `service_stats`, `score_performance`) work and paid tools are paid per call in USDC over [x402](https://x402.org). With a key, paid tools are included up to your plan's daily credits: 100 a day on a free account with a confirmed email, more on Pro, Scale and Business. Create a key at [dexra.dev/app/keys](https://dexra.dev/app/keys).

## Tools

`describe_address`, `service_stats`, `score_performance` (free); `new_launches`, `new_launches_batch`, `token_deep_check`, `token_holders_delta`, `wallet_check`, `pool_health`, `clmm_range_stats`, `market_snapshot`, `swap_preflight` (0.005–0.03 USDC per call). Details: [dexra.dev/docs](https://dexra.dev/docs).

Dexra is also in the official MCP Registry as `dev.dexra/dexra`; configs for other clients are in [dexra-dev/mcp](https://github.com/dexra-dev/mcp).

## Development

```sh
cargo build --release --target wasm32-wasip2
```

Then in Zed run `zed: install dev extension` and pick this folder.

## License

MIT
