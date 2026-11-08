[api-resource-header-start name = 'QuestionResult'; route = 'DELETE /api/v1/question-results/:id'; creditsCost = 1; api-resource-header-end]

यह रूट `QuestionResult` को id द्वारा हटाने की सुविधा प्रदान करता है।

[inline-code-attrs-start title = 'QuestionResult हटाने का cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/question-results/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'QuestionResult हटाने के अनुरोध की संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'QuestionResult हटाने के प्रतिसाद की संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** विफलता पर शामिल। **/
    reason?: string
}
[inline-code-end]