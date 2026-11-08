[api-resource-header-start name = 'Page'; route = 'GET /api/v1/pages/by-url-id'; creditsCost = 1; api-resource-header-end]

अलग-अलग पेजों को उनके संबंधित `urlId` के आधार पर प्राप्त किया जा सकता है। यह पेज शीर्षक या कमेंट की संख्या देखने के लिए उपयोगी हो सकता है।

[inline-code-attrs-start title = 'URL ID द्वारा पेज cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/by-url-id?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'URL ID द्वारा पेज रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
}
[inline-code-end]

[inline-code-attrs-start title = 'URL ID द्वारा पेज रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Included on failure. **/
    reason?: string
    page?: Page[] | null
}
[inline-code-end]

#### उपयोगी सुझाव

`urlId` जैसे मानों को URI Encode करना न भूलें।