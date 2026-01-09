Ορίστε τις απαιτούμενες μεταβλητές περιβάλλοντος:

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
```

Εκτελέστε τα τεστ:

```bash
nimble test
```

Ή τρέξτε συγκεκριμένα τεστ:

```bash
nim c -r tests/test_sso.nim
nim c -r tests/test_sso_integration.nim
```