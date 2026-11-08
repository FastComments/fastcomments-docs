[api-resource-header-start name = 'Poll'; route = 'GET /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

किसी कमेंट से जुड़े पोल को उसकी वर्तमान वोट संख्या के साथ पढ़ता है।

पोल कमेंट APIs द्वारा कमेंट पर ही भी लौटाए जाते हैं, इसलिए इसका उपयोग तब करें जब आप केवल परिणाम चाहते हों
और पूरा कमेंट नहीं।

[inline-code-attrs-start title = 'पोल प्राप्त करने का cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'पोल प्राप्त करने की रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollGetQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'पोल प्राप्त करने की रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollGetResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'poll-not-found'
    /** विफलता पर शामिल। **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

बिना पोल वाला कमेंट, डिलीट किया गया कमेंट, और कोई मौजूद न होने वाला कमेंट id — तीनों एक ही तरीके से
`poll-not-found` के साथ प्रतिक्रिया देते हैं।