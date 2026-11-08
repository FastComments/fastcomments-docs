[api-resource-header-start name = 'Tenant'; route = 'POST /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

यह रूट एकल `Tenant` जोड़ने की क्षमता प्रदान करता है।

`Tenant` बनाने के लिए निम्नलिखित प्रतिबंध हैं:

- एक `name` आवश्यक है।
- `domainConfiguration` आवश्यक है।
- `Tenant` बनाते समय निम्नलिखित मान प्रदान नहीं किए जा सकते:
  - `hasFlexPricing` 
  - `lastBillingIssueReminderDate` 
  - `flexLastBilledAmount`
- `signUpDate` भविष्य की तारीख नहीं हो सकती।
- `name` `200 characters` से लंबा नहीं हो सकता।
- `email` `300 characters` से लंबा नहीं हो सकता।
- `email` FastComments.com के सभी tenants में अद्वितीय होना चाहिए।
- यदि पैरेंट tenant के पास मान्य `TenantPackage` परिभाषित नहीं है, तो आप tenants नहीं बना सकते।
  - यदि आपका tenant FastComments.com के माध्यम से बनाया गया था, तो यह कोई समस्या नहीं होनी चाहिए।
- आप अपने पैकेज में `maxWhiteLabeledTenants` के अंतर्गत परिभाषित से अधिक tenants नहीं बना सकते।
- आपको `tenantId` क्वेरी पैरामीटर निर्दिष्ट करना होगा, जो आपके white labeling सक्षम `parent tenant` की id है।

हम कुछ ही पैरामीटर के साथ एक `Tenant` बना सकते हैं:

[inline-code-attrs-start title = 'Tenant बनाने का cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com",
    "domainConfiguration": [ { "domain": "somedomain.com" } ]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Tenant Create Request Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Tenant Create Response Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info'
    /** Included on failure. **/
    reason?: string
    tenant?: Tenant; // We return the complete created tenant on success.
}
[inline-code-end]