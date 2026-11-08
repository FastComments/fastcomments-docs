[api-resource-header-start name = 'QuestionResult'; route = 'PATCH /api/v1/question-results/:id'; creditsCost = 1; api-resource-header-end]

यह रूट एकल `QuestionResult` को अपडेट करने की क्षमता प्रदान करता है।

निम्नलिखित संरचना उन सभी मानों को दर्शाती है जिन्हें बदला जा सकता है:

[inline-code-attrs-start title = 'QuestionResult संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultPatchBody {
    urlId?: string
    anonUserId?: string
    userId?: string
    value?: string
    commentId?: string
    questionId?: string
    meta?: QuestionResultMeta[]
}
[inline-code-end]

[inline-code-attrs-start title = 'QuestionResult अपडेट cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/question-results/my-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"value": 5
}'
[inline-code-end]

[inline-code-attrs-start title = 'QuestionResult अपडेट रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'QuestionResult अपडेट रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultPatchResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'unauthorized' | 'missing-api-key' | 'missing-id' | 'not-found' | 'empty-request' | 'invalid-input'
    /** विफलता पर शामिल। **/
    reason?: string
}
[inline-code-end]