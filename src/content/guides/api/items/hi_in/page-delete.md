[api-resource-header-start name = 'Page'; route = 'DELETE /api/v1/pages/:id'; creditsCost = 1; api-resource-header-end]

यह रूट id द्वारा एकल पेज को हटाने की सुविधा प्रदान करता है।

ध्यान दें कि उसी `urlId` वाले पेज के लिए कमेंट विजेट के साथ इंटरैक्ट करने पर `Page` बिना किसी बाधा के सहजता से फिर से बन जाएगा।

[inline-code-attrs-start title = 'पेज हटाने का cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/pages/some-page-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'पेज हटाने के रिक्वेस्ट की संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'पेज हटाने के रिस्पॉन्स की संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageDeleteResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'page-does-not-exist'
    /** विफलता पर शामिल। **/
    reason?: string
}
[inline-code-end]