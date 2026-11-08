[api-resource-header-start name = 'Page'; route = 'GET /api/v1/pages'; creditsCost = 10; api-resource-header-end]

वर्तमान में आप केवल अपने खाते से जुड़े सभी पेज (या `/by-url-id` के माध्यम से एकल पेज) प्राप्त कर सकते हैं। यदि आप अधिक सूक्ष्म खोज चाहते हैं, तो [हमसे संपर्क करें](https://fastcomments.com/auth/my-account/help)।

[inline-code-attrs-start title = 'Pages cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Pages अनुरोध संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Pages प्रतिक्रिया संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** विफलता पर शामिल। **/
    reason?: string
    pages: Page[]
}
[inline-code-end]

#### उपयोगी सुझाव

`Comment` API को एक `urlId` की आवश्यकता होती है। आप पहले `Pages` API को कॉल कर सकते हैं, यह देखने के लिए कि आपके लिए उपलब्ध `urlId` मान कैसे दिखते हैं।