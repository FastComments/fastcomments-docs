必要な環境変数を設定してください:

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
```

テストを実行してください:

```bash
nimble test
```

または特定のテストを実行するには:

```bash
nim c -r tests/test_sso.nim
nim c -r tests/test_sso_integration.nim
```