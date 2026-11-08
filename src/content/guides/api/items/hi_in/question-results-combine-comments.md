[api-resource-header-start name = 'QuestionResultsCombinedWithComments'; route = 'GET /api/v1/question-results-aggregate/combine/comments'; creditsCost = 2; api-resource-header-end]

यहीं पर परिणामों और टिप्पणियों का संयोजन होता है। उदाहरण के लिए, किसी उत्पाद के लिए "हाल की सकारात्मक और नकारात्मक टिप्पणियों" का चार्ट बनाने के लिए उपयोगी।

आप मानों की एक श्रेणी (समावेशी), एक या अधिक प्रश्नों, और एक प्रारंभ तिथि (समावेशी) के आधार पर खोज कर सकते हैं।

प्रतिक्रिया की संरचना इस प्रकार है:

[inline-code-attrs-start title = 'QuestionResultsCombinedWithCommentsResponse संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SimpleComment {
    id: string
    commenterName: string
    userId?: string
    date: string
    commentHTML: string
}

interface SimpleQuestionResult {
    id: string
    value: number
}

interface CommentAndResult {
    comment: SimpleComment
    result: SimpleQuestionResult
}

interface QuestionResultsCombinedWithCommentsResponse {
    /** A date string representing when this data was calculated, since it might come from cache. **/
    createdAt: string
    results: CommentAndResult[]
}
[inline-code-end]

एग्रीगेशन के लिए उपलब्ध क्वेरी पैरामीटर यहाँ दिए गए हैं:

[inline-code-attrs-start title = 'QuestionResultsCombineComments रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsAggregateRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** You can aggregate results for one or more questions. **/
    questionId: string | string[]
    startDate?: string | number
    urlId?: string
    minValue: number
    maxValue: number
    limit?: number
    forceRecalculate?: boolean
}
[inline-code-end]

एक उदाहरण रिक्वेस्ट यहाँ है:

[inline-code-attrs-start title = 'QuestionResultsCombineComments उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results-aggregation/combine/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&questionId=some-question-id&minValue=5&maxValue=10'
[inline-code-end]

उदाहरण प्रतिक्रिया:

[inline-code-attrs-start title = 'QuestionResultsCombineComments प्रतिक्रिया उदाहरण'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
    "createdAt": "2023-08-30T00:00:00.000Z",
    "results": [
        {
            "comment": {
                "id": "some-id",
                "commentHTML": "test-2",
                "commenterName": "Test",
                "date": "2023-08-30T00:00:00.000Z",
                "userId": "some-user-id"
            },
            "result": {
                "id": "some-id",
                "value": 10
            }
        },
        {
            "comment": {
                "id": "some-id",
                "commentHTML": "test-0",
                "commenterName": "Some Name",
                "date": "2023-08-30T00:00:00.000Z",
                "userId": "some-user-id"
            },
            "result": {
                "id": "some-id",
                "value": 5
            }
        }
    ]
}
[inline-code-end]

[inline-code-attrs-start title = 'QuestionResultsCombineComments प्रतिक्रिया संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsCombineCommentsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    data: QuestionResultsCombinedWithCommentsResponse
}
[inline-code-end]

### कैशिंग और लागत संबंधी नोट्स

- जब `forceRecalculate` निर्दिष्ट किया जाता है, तो लागत सामान्य `2` के बजाय हमेशा `10` होती है।
- यदि कैश समाप्त हो जाता है और डेटा पुनर्गणना की जाती है, तो भी लागत स्थिर `2` रहती है यदि `forceRecalculate` निर्दिष्ट नहीं किया गया हो।
- यह कैश के उपयोग को प्रोत्साहित करने के लिए है।