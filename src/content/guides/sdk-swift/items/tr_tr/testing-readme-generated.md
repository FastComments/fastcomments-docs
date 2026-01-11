### Birim Testlerini Çalıştırma

Birim testleri SSO işlevselliğini kapsar:

```bash
swift test --filter SSOTests
```

### Entegrasyon Testlerini Çalıştırma

Entegrasyon testleri için ortam değişkenlerinin ayarlanması gerekir:

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
swift test --filter SSOIntegrationTests
```