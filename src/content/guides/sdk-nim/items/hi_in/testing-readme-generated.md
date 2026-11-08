आवश्यक एनवायरनमेंट वेरिएबल सेट करें:

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
```

टेस्ट चलाएँ:

```bash
nimble test
```

या विशिष्ट टेस्ट चलाएँ:

```bash
nim c -r tests/test_sso.nim
nim c -r tests/test_sso_integration.nim
```