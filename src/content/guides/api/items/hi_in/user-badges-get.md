यह एंडपॉइंट आपको विभिन्न मानदंडों के आधार पर यूज़र बैज प्राप्त करने की सुविधा देता है।

उदाहरण रिक्वेस्ट:

[inline-code-attrs-start title = 'यूज़र बैज की सूची - GET उदाहरण'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X GET "https://fastcomments.com/api/v1/user-badges?tenantId=demo&API_KEY=DEMO_API_SECRET"
[inline-code-end]

आप परिणामों को फ़िल्टर करने के लिए विभिन्न क्वेरी पैरामीटर जोड़ सकते हैं:

- `userId` - किसी विशिष्ट यूज़र के बैज प्राप्त करें
- `badgeId` - किसी विशिष्ट बैज के इंस्टेंस प्राप्त करें
- `type` - बैज टाइप के अनुसार फ़िल्टर करें (0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, आदि। पूरी सूची के लिए UserBadge संरचना देखें)
- `displayedOnComments` - बैज कमेंट्स पर दिखाया जाता है या नहीं, इसके अनुसार फ़िल्टर करें (true/false)
- `limit` - लौटाए जाने वाले बैज की अधिकतम संख्या (डिफ़ॉल्ट 30, अधिकतम 200)
- `skip` - छोड़े जाने वाले बैज की संख्या (पेजिनेशन के लिए)

उदाहरण रिस्पॉन्स:

[inline-code-attrs-start title = 'रिस्पॉन्स'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "userBadges": [
    {
      "id": "badge123",
      "userId": "user456",
      "badgeId": "badgeDef789",
      "fromTenantId": "tenant001",
      "createdAt": 1650532511000,
      "receivedAt": 1650532511000,
      "type": 14,
      "name": "Special Contributor",
      "description": "Awarded to special contributors to our community",
      "displayLabel": "Special",
      "backgroundColor": "#4a5568",
      "textColor": "#ffffff",
      "displayedOnComments": true,
      "order": 1
    },
    {
      "id": "badge124",
      "userId": "user456",
      "badgeId": "badgeDef790",
      "fromTenantId": "tenant001",
      "createdAt": 1650532598000,
      "receivedAt": 1650532598000,
      "type": 0,
      "threshold": 100,
      "name": "Centurion",
      "description": "Made 100 comments",
      "displayLabel": "100",
      "backgroundColor": "#2b6cb0",
      "textColor": "#ffffff",
      "displayedOnComments": true,
      "order": 2
    }
  ]
}
[inline-code-end]

संभावित त्रुटि रिस्पॉन्स:

[inline-code-attrs-start title = 'त्रुटि: Tenant ID अनुपस्थित'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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