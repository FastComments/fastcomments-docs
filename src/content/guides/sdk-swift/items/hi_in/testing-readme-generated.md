### यूनिट टेस्ट चलाना

यूनिट टेस्ट SSO कार्यक्षमता को कवर करते हैं:

```bash
swift test --filter SSOTests
```

### इंटीग्रेशन टेस्ट चलाना

इंटीग्रेशन टेस्ट के लिए एनवायरनमेंट वेरिएबल सेट करना आवश्यक है:

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
swift test --filter SSOIntegrationTests
```