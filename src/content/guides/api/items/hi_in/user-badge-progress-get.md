यह एंडपॉइंट आपको विभिन्न मानदंडों के आधार पर यूज़र बैज प्रोग्रेस रिकॉर्ड्स प्राप्त करने की अनुमति देता है।

उदाहरण रिक्वेस्ट:

[inline-code-attrs-start title = 'बैज प्रोग्रेस लिस्ट करें - GET उदाहरण'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X GET "https://fastcomments.com/api/v1/user-badge-progress?tenantId=demo&API_KEY=DEMO_API_SECRET"
[inline-code-end]

आप परिणामों को फ़िल्टर करने के लिए विभिन्न क्वेरी पैरामीटर जोड़ सकते हैं:

- `userId` - किसी विशिष्ट यूज़र की प्रोग्रेस प्राप्त करें
- `limit` - लौटाए जाने वाले रिकॉर्ड्स की अधिकतम संख्या (डिफ़ॉल्ट 30, अधिकतम 200)
- `skip` - छोड़े जाने वाले रिकॉर्ड्स की संख्या (पेजिनेशन के लिए)

उदाहरण रिस्पॉन्स:

[inline-code-attrs-start title = 'रिस्पॉन्स'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "userBadgeProgresses": [
    {
      "id": "progress123",
      "tenantId": "tenant001",
      "userId": "user456",
      "firstCommentId": "comment789",
      "firstCommentDate": 1650532511000,
      "autoTrustFactor": 0.75,
      "progress": {
        "0": 42,
        "1": 120,
        "2": 15,
        "3": 3,
        "5": 5,
        "6": 1800000,
        "8": 0,
        "7": 0
      }
    },
    {
      "id": "progress124",
      "tenantId": "tenant001",
      "userId": "user789",
      "firstCommentId": "comment790",
      "firstCommentDate": 1650532598000,
      "autoTrustFactor": 0.5,
      "progress": {
        "0": 12,
        "1": 15,
        "2": 4
      }
    }
  ]
}
[inline-code-end]

संभावित त्रुटि रिस्पॉन्स:

[inline-code-attrs-start title = 'त्रुटि: Tenant ID गायब है'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-tenant-id",
  "reason": "Tenant ID (query param: tenantId) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = 'त्रुटि: अमान्य Limit'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "invalid-limit",
  "reason": "The limit (query param: limit) is too large (> 200)."
}
[inline-code-end]