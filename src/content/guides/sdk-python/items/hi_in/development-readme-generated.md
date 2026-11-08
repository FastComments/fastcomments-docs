### टेस्ट चलाना

```bash
# एनवायरनमेंट वेरिएबल सेट करें
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"

# टेस्ट चलाएँ
pytest tests/
```

### क्लाइंट को पुनः जनरेट करना

नवीनतम OpenAPI स्पेसिफिकेशन से API क्लाइंट को पुनः जनरेट करने के लिए:

```bash
./update.sh
```

यह करेगा:
1. चल रहे FastComments सर्वर से नवीनतम OpenAPI स्पेक डाउनलोड करेगा (या लोकल openapi.yaml का उपयोग करेगा)
2. Python क्लाइंट कोड जनरेट करेगा
3. डायरेक्टरी संरचना को फ्लैट करेगा
4. अनावश्यक कॉन्फ़िगरेशन फ़ाइलों को साफ़ करेगा