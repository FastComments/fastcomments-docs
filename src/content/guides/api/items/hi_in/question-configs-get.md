[api-resource-header-start name = 'QuestionConfig'; route = 'GET /api/v1/question-configs'; creditsCost = 1; api-resource-header-end]

यह रूट एक बार में 100 तक `QuestionConfig` ऑब्जेक्ट लौटाता है, पृष्ठांकित (paginated) रूप में। प्रत्येक 100 ऑब्जेक्ट की लागत 1 है। इन्हें प्रश्न पाठ के आरोही क्रम (`question` फ़ील्ड) में क्रमबद्ध किया जाता है।

[inline-code-attrs-start title = 'QuestionConfig उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-configs?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'QuestionConfig अनुरोध संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigRequestByIdQueryParams {
    tenantId: string
    API_KEY: string
    /** पृष्ठांकन के लिए। 0 से शुरू होता है। **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'QuestionConfig प्रतिक्रिया संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigByIdResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** विफलता पर शामिल। **/
    reason?: string
    questionConfigs: QuestionConfig[]
}
[inline-code-end]