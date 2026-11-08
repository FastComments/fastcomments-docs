[api-resource-header-start name = 'Tenant'; route = 'PATCH /api/v1/tenants/:id'; creditsCost = 1; api-resource-header-end]

यह API एंडपॉइंट `id` द्वारा एक `Tenant` को अपडेट करने की क्षमता प्रदान करता है।

एक `Tenant` को अपडेट करने की निम्नलिखित सीमाएँ हैं:

- निम्नलिखित मान अपडेट नहीं किए जा सकते:
  - `hasFlexPricing` 
  - `lastBillingIssueReminderDate` 
  - `flexLastBilledAmount`
  - `managedByTenantId`
- `signUpDate` भविष्य में नहीं हो सकता।
- `name` `200 characters` से लंबा नहीं हो सकता।
- `email` `300 characters` से लंबा नहीं हो सकता।
- `email` FastComments.com के सभी tenants में अद्वितीय होना चाहिए।
- `billingInfoValid` को `true` पर सेट करते समय, उसी अनुरोध में `billingInfo` प्रदान किया जाना चाहिए।
- आप अपने स्वयं के tenant से जुड़े `packageId` को अपडेट नहीं कर सकते।
- आप अपने स्वयं के tenant से जुड़े `paymentFrequency` को अपडेट नहीं कर सकते।

[inline-code-attrs-start title = 'Tenant PATCH cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenants/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Tenant PATCH अनुरोध संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Tenant PATCH प्रतिक्रिया संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPatchResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info'; 
    /** विफलता पर शामिल। **/
    reason?: string
}
[inline-code-end]