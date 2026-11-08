यह एंडपॉइंट आपको यूज़र बैज असाइनमेंट अपडेट करने की अनुमति देता है।

वर्तमान में, केवल `displayedOnComments` प्रॉपर्टी अपडेट की जा सकती है, जो यह नियंत्रित करती है कि बैज यूज़र की कमेंट्स पर दिखाया जाए या नहीं।

उदाहरण रिक्वेस्ट:

[inline-code-attrs-start title = 'PUT रिक्वेस्ट उदाहरण'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X PUT "https://fastcomments.com/api/v1/user-badges/badge123?tenantId=demo&API_KEY=DEMO_API_SECRET" \
-H "Content-Type: application/json" \
-d '{
  "displayedOnComments": false
}'
[inline-code-end]

उदाहरण रिस्पॉन्स:

[inline-code-attrs-start title = 'रिस्पॉन्स'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success"
}
[inline-code-end]

संभावित एरर रिस्पॉन्स:

[inline-code-attrs-start title = 'एरर: Tenant ID गायब है'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-tenant-id",
  "reason": "Tenant ID (query param: tenantId) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = 'एरर: ID गायब है'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-id",
  "reason": "The User Badge ID (url param: id) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = 'एरर: नहीं मिला'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "not-found",
  "reason": "The user badge badge123 was not found."
}
[inline-code-end]