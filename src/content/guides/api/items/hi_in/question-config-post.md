[api-resource-header-start name = 'QuestionConfig'; route = 'POST /api/v1/question-configs'; creditsCost = 1; api-resource-header-end]

यह API एंडपॉइंट एक `QuestionConfig` बनाने की क्षमता प्रदान करता है।

[inline-code-attrs-start title = 'QuestionConfig POST cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/question-configs?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some name that shows on reports.",
    "question": "how much do you like this api?",
    "type": 'slider',
    "reportingOrder": 0,
    "min": 0,
    "max": 10
}'
[inline-code-end]

[inline-code-attrs-start title = 'QuestionConfig POST अनुरोध संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'QuestionConfig POST प्रतिक्रिया संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface QuestionConfigPostResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'internal' | 'invalid-input';  
    /** विफलता पर शामिल। **/
    reason?: string
    /** बनाया गया ऑब्जेक्ट। **/
    questionConfig?: QuestionConfig
}
[inline-code-end]