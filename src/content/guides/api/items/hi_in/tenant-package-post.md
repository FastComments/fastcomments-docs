[api-resource-header-start name = 'TenantPackage'; route = 'POST /api/v1/tenant-packages'; creditsCost = 1; api-resource-header-end]

यह रूट एक सिंगल `TenantPackage` जोड़ने की सुविधा प्रदान करता है।

एक `TenantPackage` बनाने के लिए निम्नलिखित प्रतिबंध हैं:

- निम्नलिखित पैरामीटर आवश्यक हैं:
    - `name`
    - `tenantId`
    - `monthlyCostUSD` - null हो सकता है।
    - `yearlyCostUSD` - null हो सकता है।
    - `maxMonthlyPageLoads`
    - `maxMonthlyAPICredits`
    - `maxMonthlyComments`
    - `maxConcurrentUsers`
    - `maxTenantUsers`
    - `maxSSOUsers`
    - `maxModerators`
    - `maxDomains`
    - `hasDebranding`
    - `forWhoText`
    - `featureTaglines`
    - `hasFlexPricing` - यदि true है, तो सभी `flex*` पैरामीटर आवश्यक हैं।
- `name` की लंबाई `50 characters` से अधिक नहीं हो सकती।
- प्रत्येक `forWhoText` आइटम की लंबाई `200 characters` से अधिक नहीं हो सकती।
- प्रत्येक `featureTaglines` आइटम की लंबाई `100 characters` से अधिक नहीं हो सकती।
- `TenantPackage` पैरेंट टेनेंट से "छोटा" होना चाहिए। उदाहरण के लिए, सभी `max*` पैरामीटर के मान पैरेंट टेनेंट से कम होने चाहिए।
- एक व्हाइट लेबल वाले टेनेंट में **अधिकतम पाँच पैकेज** हो सकते हैं।
- केवल व्हाइट लेबलिंग एक्सेस वाले टेनेंट ही `TenantPackage` बना सकते हैं।
- आप अपने ही टेनेंट में पैकेज नहीं जोड़ सकते। :)

हम एक `TenantPackage` इस प्रकार बना सकते हैं:

[inline-code-attrs-start title = 'TenantPackage Create via Email cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-packages?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
  "name": "Default Package",
  "tenantId": "some-child-tenant-id",
  "monthlyCostUSD": null,
  "yearlyCostUSD": null,
  "maxMonthlyPageLoads": 50000,
  "maxMonthlyAPICredits": 50000,
  "maxMonthlyComments": 50000,
  "maxConcurrentUsers": 50000,
  "maxTenantUsers": 10,
  "maxSSOUsers": 50000,
  "maxModerators": 100,
  "maxDomains": 3,
  "hasWhiteLabeling": false,
  "hasDebranding": true,
  "forWhoText": "For Everyone",
  "featureTaglines": [
    "Some Tag",
    "Some Other Tag"
  ],
  "hasFlexPricing": true,
  "flexPageLoadCostCents": 100,
  "flexPageLoadUnit": 100000,
  "flexCommentCostCents": 100,
  "flexCommentUnit": 100000,
  "flexSSOUserCostCents": 100,
  "flexSSOUserUnit": 1000,
  "flexAPICreditCostCents": 100,
  "flexAPICreditUnit": 50000,
  "flexModeratorCostCents": 500,
  "flexModeratorUnit": 1,
  "flexAdminCostCents": 1000,
  "flexAdminUnit": 1,
  "flexDomainCostCents": 1000,
  "flexDomainUnit": 1,
  "flexMinimumCostCents": 99
}'
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage Create Request Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage Create Response Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagePostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' 'white-labeling-not-allowed' | 'name-too-long' | 'for-who-text-too-long' | 'feature-tag-lines-too-long' | 'no-package' | 'invalid-package' | 'unauthorized' | 'child-tenant-too-large' | 'flex-param-missing' | 'unexpected-flex-param' | 'package-limit-reached' | 'flex-param-missing' | 'unexpected-flex-param'
    /** Included on failure. **/
    reason?: string
    tenantPackage?: TenantPackage; // सफलता पर हम पूरा बनाया गया टेनेंट पैकेज लौटाते हैं।
}
[inline-code-end]