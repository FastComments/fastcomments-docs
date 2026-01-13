### Покретање јединичних тестова

Јединични тестови обухватају SSO функционалност:

```bash
swift test --filter SSOTests
```

### Покретање интеграционих тестова

Интеграциони тестови захтевају да су променљиве окружења постављене:

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
swift test --filter SSOIntegrationTests
```