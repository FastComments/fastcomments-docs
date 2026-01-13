### Покретање јединичних тестова

Јединични тестови покривају SSO функционалност:

```bash
swift test --filter SSOTests
```

### Покретање интеграционих тестова

Интеграциони тестови захтијевају да су постављене променљиве окружења:

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
swift test --filter SSOIntegrationTests
```