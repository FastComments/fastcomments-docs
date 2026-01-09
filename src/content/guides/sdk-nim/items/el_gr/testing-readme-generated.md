Ορίστε τις απαραίτητες μεταβλητές περιβάλλοντος:

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
```

Εκτελέστε τις δοκιμές:

```bash
nimble test
```

Ή εκτελέστε συγκεκριμένες δοκιμές:

```bash
nim c -r tests/test_sso.nim
nim c -r tests/test_sso_integration.nim
```