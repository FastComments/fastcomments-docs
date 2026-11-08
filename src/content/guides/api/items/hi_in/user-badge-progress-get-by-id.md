यह एंडपॉइंट आपको अपनी यूनिक ID द्वारा किसी विशिष्ट यूज़र बैज प्रोग्रेस रिकॉर्ड को फ़ेच करने की अनुमति देता है।

उदाहरण रिक्वेस्ट:

[inline-code-attrs-start title = 'ID द्वारा बैज प्रोग्रेस प्राप्त करें - GET उदाहरण'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X GET "https://fastcomments.com/api/v1/user-badge-progress/progress123?tenantId=demo&API_KEY=DEMO_API_SECRET"
[inline-code-end]

उदाहरण रिस्पॉन्स:

[inline-code-attrs-start title = 'रिस्पॉन्स'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "userBadgeProgress": {
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
  }
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

[inline-code-attrs-start title = 'त्रुटि: नहीं मिला'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "not-found",
  "reason": "The user badge progress progress123 was not found."
}
[inline-code-end]