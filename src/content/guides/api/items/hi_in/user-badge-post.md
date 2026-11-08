यह एंडपॉइंट आपको एक नया यूज़र बैज असाइनमेंट बनाने की अनुमति देता है।

अनुरोध उदाहरण:

[inline-code-attrs-start title = 'POST अनुरोध उदाहरण'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X POST "https://fastcomments.com/api/v1/user-badges?tenantId=demo&API_KEY=DEMO_API_SECRET" \
-H "Content-Type: application/json" \
-d '{
  "userId": "user456",
  "badgeId": "badgeDef789",
  "displayedOnComments": true
}'
[inline-code-end]

अनुरोध बॉडी में निम्नलिखित पैरामीटर होने चाहिए:

- `userId` (आवश्यक) - उस यूज़र की ID जिसे बैज असाइन किया जाना है
- `badgeId` (आवश्यक) - असाइन किए जाने वाले बैज की ID
- `displayedOnComments` (वैकल्पिक) - क्या बैज यूज़र की टिप्पणियों पर प्रदर्शित होना चाहिए (डिफ़ॉल्ट true)

महत्वपूर्ण नोट्स:
1. बैज आपके टेनेंट के बैज कैटलॉग में मौजूद और सक्रिय होना चाहिए
2. आप केवल उन यूज़र्स को बैज असाइन कर सकते हैं जो आपके टेनेंट से संबंधित हैं या जिन्होंने आपकी साइट पर टिप्पणी की है

प्रतिक्रिया उदाहरण:

[inline-code-attrs-start title = 'प्रतिक्रिया'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

संभावित त्रुटि प्रतिक्रियाएँ:

[inline-code-attrs-start title = 'त्रुटि: Tenant ID अनुपस्थित'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-tenant-id",
  "reason": "Tenant ID (query param: tenantId) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = 'त्रुटि: User ID अनुपस्थित'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-user-id",
  "reason": "User ID (body param: userId) is required."
}
[inline-code-end]

[inline-code-attrs-start title = 'त्रुटि: Badge ID अनुपस्थित'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-badge-id",
  "reason": "Badge ID (body param: badgeId) is required."
}
[inline-code-end]

[inline-code-attrs-start title = 'त्रुटि: बैज नहीं मिला'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "badge-not-found",
  "reason": "The badge badgeDef789 was not found or is not enabled."
}
[inline-code-end]

[inline-code-attrs-start title = 'त्रुटि: अनधिकृत यूज़र'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "unauthorized-user",
  "reason": "You can only add badges to users who belong to your tenant or have commented on your site."
}
[inline-code-end]