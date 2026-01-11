### Εκτέλεση δοκιμών μονάδας

Οι δοκιμές μονάδας καλύπτουν τη λειτουργικότητα SSO:

```bash
swift test --filter SSOTests
```

### Εκτέλεση δοκιμών ολοκλήρωσης

Οι δοκιμές ολοκλήρωσης απαιτούν να οριστούν μεταβλητές περιβάλλοντος:

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
swift test --filter SSOIntegrationTests
```