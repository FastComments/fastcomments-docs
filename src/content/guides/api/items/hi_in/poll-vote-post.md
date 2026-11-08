[api-resource-header-start name = 'PollVote'; route = 'POST /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

पोल पर एक वोट रिकॉर्ड करता है।

एक वोटर के पास प्रति पोल अधिकतम एक वोट होता है। उसी वोटर के लिए इसे दोबारा कॉल करने पर दूसरा वोट जोड़ने के बजाय उनका वोट नए विकल्प पर चला जाता है, और उसी विकल्प के लिए वोट करने पर कुछ नहीं होता।

प्रतिक्रिया में पोल शामिल होता है, इसलिए आपको दूसरा अनुरोध किए बिना अपडेट की गई गिनती मिल जाती है।

[inline-code-attrs-start title = 'PollVote Create cURL उदाहरण'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"commentId": "comment-id",
	"optionId": "the-option-id",
	"userId": "user-id"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Anonymous PollVote Create cURL उदाहरण'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"commentId": "comment-id",
	"optionId": "the-option-id",
	"anonUserId": "some-randomly-generated-identifier",
	"ip": "203.0.113.4"
}'
[inline-code-end]

[inline-code-attrs-start title = 'PollVote Create अनुरोध संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteCreateQueryParams {
    tenantId: string
    API_KEY: string
}

interface PollVoteCreateBody {
    commentId: string
    optionId: string
    /** userId या anonUserId में से एक आवश्यक है। **/
    userId?: string
    anonUserId?: string
    /** अंतिम उपयोगकर्ता का IP, जो गुमनाम रेट लिमिट के लिए उपयोग होता है। डिफ़ॉल्ट रूप से कॉलर का IP। **/
    ip?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'PollVote Create प्रतिक्रिया संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteCreateResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-comment-id' | 'missing-user-id' | 'invalid-user' | 'unauthorized' | 'poll-not-found' | 'poll-invalid-option' | 'poll-closed' | 'poll-login-required' | 'rate-limited'
    /** विफलता पर शामिल। **/
    reason?: string
    pollVote: PollVote
    /** अपडेट की गई गिनती के साथ पोल। **/
    poll: CommentPoll
}
[inline-code-end]

### गुमनाम वोट

`userId` के स्थान पर `anonUserId` सेट करें ताकि किसी ऐसे व्यक्ति के लिए वोट रिकॉर्ड किया जा सके जो लॉग इन नहीं है। वह आईडी कहीं भी किसी उपयोगकर्ता से मेल खाना ज़रूरी नहीं है - यह केवल सत्र की पहचान करता है, ताकि एक ही व्यक्ति दो बार गिना न जाए।

गुमनाम वोटिंग आपकी साइट के लिए सक्षम होनी चाहिए। यदि वोटिंग केवल लॉग इन उपयोगकर्ताओं तक सीमित है, तो केवल `anonUserId` वाला वोट `poll-login-required` के साथ विफल हो जाता है।

गुमनाम वोट भी प्रति IP प्रति पोल रेट सीमित होते हैं, ताकि कोई व्यक्ति अपना सत्र साफ़ करके पोल को भर न सके। अंतिम उपयोगकर्ता का `ip` भेजें ताकि वह सीमा आपके सर्वर के बजाय उस पर लागू हो।

### अन्य नोट्स

- `userId` आपकी साइट पर मौजूद किसी उपयोगकर्ता का होना चाहिए। दूसरी साइट के उपयोगकर्ता के लिए वोट अस्वीकार कर दिए जाते हैं।
- बंद पोल पर वोट करना `poll-closed` के साथ विफल हो जाता है।
- यह API पोल पर गिनती अपडेट करता है और उन्हें कनेक्टेड विजेट्स पर लाइव भेजता है।