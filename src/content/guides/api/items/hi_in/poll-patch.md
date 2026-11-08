[api-resource-header-start name = 'Poll'; route = 'PATCH /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

वोट को बिना बाधित किए पोल को संपादित करता है। इसका उपयोग प्रश्न या किसी विकल्प में टाइपो ठीक करने, पोल को बंद या फिर से खोलने, या यह बदलने के लिए करें कि कौन देख सकता है कि किसने वोट दिया।

विकल्पों को उनके `id` से संबोधित किया जाता है, और `PATCH` उन विकल्पों का नाम बदल देता है जिन्हें आप नामित करते हैं। विकल्प जोड़ने, हटाने या क्रम बदलने के लिए, पूरी विकल्प सूची `PUT /api/v1/polls/:commentId` पर भेजें: जिन विकल्पों को आप उनकी ids के साथ भेजते हैं, वे वहीं अपने वोट बरकरार रखते हैं।

हर फ़ील्ड वैकल्पिक है, लेकिन कम से कम एक देना अनिवार्य है।

[inline-code-attrs-start title = 'Poll Patch cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "Which release should we cut first?",
	"options": [{"id": "the-option-id", "label": "The bugfix release"}]
}'
[inline-code-end]

[inline-code-attrs-start title = 'पोल अभी बंद करें cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"closesAt": "2020-01-01T00:00:00.000Z"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Poll Patch अनुरोध संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPatchQueryParams {
    tenantId: string
    API_KEY: string
}

interface PollPatchBody {
    question?: string | null
    /** मौजूदा विकल्पों का नाम बदलता है। दी गई हर id पोल पर पहले से मौजूद होनी चाहिए। **/
    options?: { id: string, label: string }[] | null
    /** अतीत की तारीख पोल को अभी बंद कर देती है। null बंद पोल को फिर से खोल देता है। **/
    closesAt?: string | null
    /** 0 अनाम, 1 एडमिन और मॉडरेटर, 2 सभी। **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = 'Poll Patch प्रतिक्रिया संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPatchResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'poll-not-found' | 'poll-invalid' | 'poll-privacy-locked' | 'locked'
    /** विफलता पर शामिल। **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### अन्य नोट्स

- पोल पर मौजूद नहीं किसी विकल्प की id नामित करने पर चुपचाप कुछ न करने के बजाय `poll-invalid` के साथ विफल होता है।
- लेबल पोल के भीतर अद्वितीय रहने चाहिए, जिन विकल्पों को आप नहीं बदल रहे हैं उन्हें भी गिनते हुए।
- पोल बनाने के विपरीत, यहाँ `closesAt` अतीत में हो सकता है - इसी तरह आप पोल को तुरंत बंद करते हैं।
- पोल की गोपनीयता को संकुचित किया जा सकता है लेकिन वोट होने के बाद बढ़ाया नहीं जा सकता।
- लॉक किए गए कमेंट का पोल नहीं बदला जा सकता, और यह `locked` के साथ विफल होता है।