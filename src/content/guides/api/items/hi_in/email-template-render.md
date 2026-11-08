[api-resource-header-start name = 'EmailTemplate'; route = 'POST /api/v1/email-templates/render'; creditsCost = 1; api-resource-header-end]

यह API एंडपॉइंट ईमेल टेम्पलेट्स का प्रीव्यू देखने की सुविधा प्रदान करता है।

[inline-code-attrs-start title = 'न्यूनतम EmailTemplate प्रीव्यू POST cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/email-templates/render?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "emailTemplateId": "comment-user-mention",
    "displayName": "I'm a custom template.",
    "ejs": "This is an @mention notification! My name is <%= comment.commenterName %>."
}'
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate POST अनुरोध संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate POST प्रतिक्रिया संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface EmailTemplatePostResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'unexpected-param' | 'does-not-render';
    /** विफलता पर शामिल। **/
    reason?: string
    /** सफलता पर रेंडर किया गया HTML। **/
    html?: string
}
[inline-code-end]