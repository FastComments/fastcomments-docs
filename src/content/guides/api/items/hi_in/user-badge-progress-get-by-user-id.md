यह एंडपॉइंट आपको यूज़र ID द्वारा किसी यूज़र का बैज प्रोग्रेस रिकॉर्ड प्राप्त करने की अनुमति देता है।

उदाहरण रिक्वेस्ट:

[inline-code-attrs-start title = 'यूज़र ID द्वारा बैज प्रोग्रेस प्राप्त करें - GET उदाहरण'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X GET "https://fastcomments.com/api/v1/user-badge-progress/user/user456?tenantId=demo&API_KEY=DEMO_API_SECRET"
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

संभावित एरर रिस्पॉन्स:

[inline-code-attrs-start title = 'एरर: टेनेंट ID अनुपस्थित'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-tenant-id",
  "reason": "Tenant ID (query param: tenantId) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = 'एरर: यूज़र ID अनुपस्थित'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-user-id",
  "reason": "The User ID (path param: userId) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = 'एरर: नहीं मिला'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "not-found",
  "reason": "No badge progress found for user user456 in tenant tenant001."
}
[inline-code-end]