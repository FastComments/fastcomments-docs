[api-resource-header-start name = 'TenantPackage'; route = 'PATCH /api/v1/tenant-packages/:id'; creditsCost = 1; api-resource-header-end]

यह API एंडपॉइंट `id` द्वारा एक `TenantPackage` को अपडेट करने की क्षमता प्रदान करता है।

एक `TenantPackage` को अपडेट करने की निम्नलिखित पाबंदियाँ हैं:

- यदि आप `hasFlexPricing` को true सेट कर रहे हैं, तो उसी अनुरोध में सभी `flex*` पैरामीटर आवश्यक हैं।
- `name` की लंबाई `50 characters` से अधिक नहीं हो सकती।
- प्रत्येक `forWhoText` आइटम की लंबाई `200 characters` से अधिक नहीं हो सकती।
- प्रत्येक `featureTaglines` आइटम की लंबाई `100 characters` से अधिक नहीं हो सकती।
- `TenantPackage` पैरेंट टेनेंट से "छोटा" होना चाहिए। उदाहरण के लिए, सभी `max*` पैरामीटर के मान पैरेंट टेनेंट से कम होने चाहिए।
- आप `TenantPackage` से जुड़े `tenantId` को नहीं बदल सकते।

[inline-code-attrs-start title = 'TenantPackage PATCH cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-packages/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage PATCH अनुरोध संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage PATCH प्रतिक्रिया संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPackagePatchResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'white-labeling-not-allowed' | 'name-too-long' | 'for-who-text-too-long' | 'feature-tag-lines-too-long' | 'no-package' | 'invalid-package' | 'unauthorized' | 'child-tenant-too-large' | 'flex-param-missing' | 'unexpected-flex-param' | 'package-limit-reached' | 'flex-param-missing' | 'unexpected-flex-param'; 
    /** विफलता पर शामिल। **/
    reason?: string
}
[inline-code-end]