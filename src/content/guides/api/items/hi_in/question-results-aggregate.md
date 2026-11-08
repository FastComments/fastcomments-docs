[api-resource-header-start name = 'QuestionResultsAggregate'; route = 'GET /api/v1/question-results-aggregate'; creditsCost = 2; api-resource-header-end]

यहीं पर परिणामों का समुच्चयन (aggregation) होता है।

समुच्चयन प्रतिक्रिया की संरचना इस प्रकार है:

[inline-code-attrs-start title = 'QuestionResultsAggregationResult संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultDataPoint {
    /** वर्तमान डेटा पॉइंट (तिथि बकेट या पेज) के लिए किसी मान की घटनाओं की संख्या का मान-से-गिनती मैप। **/
    v: Map<ValueAsString, number>
    total: number
}

interface QuestionResultsAggregationResult {
    /** ध्यान दें - timeBucket निर्दिष्ट न होने पर null होता है। **/
    dataByDateBucket?: Map<DateString, QuestionResultDataPoint>
    dataByUrlId?: Map<URLIdString, QuestionResultDataPoint>
    countsByValue?: Map<ValueAsString, number>
    /** समुच्चयित किए गए परिणामों की कुल संख्या। **/
    total: number
    /** परिणामी भारित औसत। यह एक float है, आमतौर पर दो दशमलव या उससे कम। **/
    average: number
    /** एक तिथि स्ट्रिंग जो दर्शाती है कि यह डेटा कब गणना किया गया था, क्योंकि यह कैश से आ सकता है। **/
    createdAt: string
}
[inline-code-end]

समुच्चयन के लिए उपलब्ध क्वेरी पैरामीटर यहाँ दिए गए हैं:

[inline-code-attrs-start title = 'QuestionResultsAggregation रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsAggregateRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** आप एक या अधिक प्रश्नों के लिए परिणाम समुच्चयित कर सकते हैं। **/
    questionId: string | string[]
    startDate?: string | number
    timeBucket?: 'day' | 'month' | 'year'
    /** किसी विशिष्ट पेज के लिए समुच्चयन। **/
    urlId?: string
    /** किसी विशिष्ट उपयोगकर्ता के लिए समुच्चयन। **/
    userId?: string
    /** अभी पुनः गणना करें और कैश अपडेट करें। **/
    forceRecalculate?: boolean
}
[inline-code-end]

एक उदाहरण रिक्वेस्ट यहाँ है:

[inline-code-attrs-start title = 'QuestionResultsAggregation उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results-aggregation?tenantId=demo&API_KEY=DEMO_API_SECRET&questionId=some-question-id'
[inline-code-end]

उदाहरण प्रतिक्रिया:

[inline-code-attrs-start title = 'QuestionResultsAggregation प्रतिक्रिया उदाहरण'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    {
        "average": 8.33,
        "countsByValue": {
            "5": 1,
            "10": 2
        },
        "createdAt": "2023-08-30T00:00:00.000Z",
        "dataByUrlId": {
            "some-page": {
                "total": 3,
                "v": {
                    "5": 1,
                    "10": 2
                }
            }
        },
        "total": 3
    }
[inline-code-end]

[inline-code-attrs-start title = 'QuestionResultsAggregation प्रतिक्रिया संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsAggregationResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** विफलता पर शामिल। **/
    reason?: string
    data: QuestionResultsAggregationResult
}
[inline-code-end]

### परफ़ॉर्मेंस नोट्स

- कैश मिस की स्थिति में समुच्चयन में आमतौर पर प्रति मिलियन परिणाम पाँच सेकंड लगते हैं।
- अन्यथा, रिक्वेस्ट स्थिर-समय (constant-time) में होती हैं।

### कैशिंग और लागत नोट्स

- जब `forceRecalculate` निर्दिष्ट किया जाता है, तो लागत सामान्य `2` के बजाय हमेशा `10` होती है।
- यदि कैश समाप्त हो जाता है और डेटा पुनः गणना किया जाता है, तो भी लागत एक स्थिर `2` रहती है यदि `forceRecalculate` निर्दिष्ट नहीं किया गया है। कैश समुच्चयित किए गए डेटा सेट के आकार के आधार पर समाप्त होता है (30 सेकंड से 5 मिनट के बीच भिन्न हो सकता है)।
- यह कैश के उपयोग को प्रोत्साहित करने के लिए है।