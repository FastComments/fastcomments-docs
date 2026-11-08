यह एंडपॉइंट आपको अपनी अद्वितीय ID द्वारा एक विशिष्ट यूज़र बैज प्राप्त करने की अनुमति देता है।

उदाहरण रिक्वेस्ट:

[inline-code-attrs-start title = 'ID द्वारा यूज़र बैज प्राप्त करें - GET उदाहरण'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X GET "https://fastcomments.com/api/v1/user-badges/badge123?tenantId=demo&API_KEY=DEMO_API_SECRET"
[inline-code-end]

उदाहरण रिस्पॉन्स:

[inline-code-attrs-start title = 'रिस्पॉन्स'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "userBadge": {
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
  "reason": "The user badge badge123 was not found."
}
[inline-code-end]