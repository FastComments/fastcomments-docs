[api-resource-header-start name = 'QuestionResults'; route = 'GET /api/v1/question-results'; creditsCost = 1; api-resource-header-end]

यह रूट एक बार में 1000 तक `QuestionResults` ऑब्जेक्ट लौटाता है, पृष्ठांकित (paginated) रूप में। प्रत्येक 100 ऑब्जेक्ट पर लागत 1 है। इन्हें `createdAt` के अनुसार आरोही क्रम में सॉर्ट किया जाता है। आप विभिन्न पैरामीटर के आधार पर फ़िल्टर कर सकते हैं।

[inline-code-attrs-start title = 'QuestionResults उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'QuestionResults रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** पृष्ठांकन के लिए। 0 से शुरू होता है। **/
    skip?: number
    /** पृष्ठांकन के लिए। **/
    limit?: number
    /** किसी विशिष्ट पेज के परिणाम प्राप्त करें। **/
    urlId?: string
    /** किसी विशिष्ट उपयोगकर्ता के परिणाम प्राप्त करें। **/
    userId?: string
    startDate?: string | number
    questionId?: string | string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'QuestionResults रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsResponse {
    status: 'success' | 'failed'
    /** विफलता की स्थिति में शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** विफलता की स्थिति में शामिल। **/
    reason?: string
    questionResults: QuestionResults[]
}
[inline-code-end]