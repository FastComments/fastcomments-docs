[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

यह API id द्वारा एकल कमेंट प्राप्त करने की क्षमता प्रदान करती है।

[inline-code-attrs-start title = 'Comments Get-By-Id cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments/some-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Comment Get-By-Id रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsGetByIdQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Comments Get-By-Id रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentGetByIdResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'missing-id'
    /** विफलता पर शामिल। **/
    reason?: string
    /** कमेंट! **/
    comment?: Comment
}
[inline-code-end]