[api-resource-header-start name = 'Poll'; route = 'PUT /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

किसी मौजूदा कमेंट पर एक पोल जोड़ता है, या उस पोल की पूरी स्थिति सेट करता है जो पहले से मौजूद है।

बॉडी पूरा पोल होता है, और आपके द्वारा भेजे गए विकल्प उसी क्रम में पोल के विकल्प बन जाते हैं। प्रत्येक विकल्प
उसके `id` से मैच किया जाता है:

- किसी मौजूदा विकल्प के `id` के साथ भेजा गया विकल्प उस विकल्प और उसके वोटों को बनाए रखता है। उसका लेबल और स्थान
  आपके द्वारा भेजे गए मानों में अपडेट हो जाता है।
- बिना `id` के भेजा गया विकल्प जोड़ा जाता है, बिना किसी वोट के।
- कोई मौजूदा विकल्प जिसे आप छोड़ देते हैं, वह हटा दिया जाता है, उस पर डाले गए वोटों सहित। `totalVotes` उतनी ही
  मात्रा से घट जाता है।

इसलिए एक विकल्प जोड़ने के लिए, मौजूदा विकल्पों को उनके ids के साथ भेजें और नए विकल्प को बिना id के भेजें। एक को
हटाने के लिए, सूची उसे छोड़कर भेजें। विकल्प ids `GET /api/v1/polls/:commentId` द्वारा लौटाए गए पोल पर होते हैं।

बिल्कुल भी ids न भेजने पर हर विकल्प बदल जाता है और पोल पर पहले से डाले गए हर वोट को हटा दिया जाता है। यदि पोल पर
वोट हैं, तो इसके लिए `replaceVotes=true` आवश्यक है, और इसके बिना API `replace-votes-required` के साथ प्रतिक्रिया देता है।

अन्य फ़ील्ड भी बदल दिए जाते हैं: `closesAt`, `privacy` या `requireVoteToSeeResults` को छोड़ने पर वे उनके डिफ़ॉल्ट पर
रीसेट हो जाते हैं। केवल एक फ़ील्ड बदलने और बाकी को वैसे ही छोड़ने के लिए, `PATCH /api/v1/polls/:commentId` का उपयोग करें।

[inline-code-attrs-start title = 'Poll Put cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PUT \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "Which release should we cut first?",
	"options": [
		{"id": "existing-option-id", "label": "The bugfix release"},
		{"label": "The feature release"}
	],
	"closesAt": "2026-12-31T00:00:00.000Z"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Poll Put रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutQueryParams {
    tenantId: string
    API_KEY: string
    /** Required to keep none of the existing option ids when the poll has votes, since that deletes them all. **/
    replaceVotes?: boolean
}

interface PollPutOption {
    /** The id of an existing option, to keep it and its votes. Omit to add a new option. **/
    id?: string | null
    label: string
}

interface PollPutBody {
    question: string
    /** The complete, ordered list. Existing options left out are removed with their votes. **/
    options: PollPutOption[]
    /** Must be in the future when the comment has no poll yet. Omit for a poll that stays open. **/
    closesAt?: string | null
    /** 0 anonymous (the default), 1 admins and moderators, 2 everyone. **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = 'Poll Put रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'polls-disabled' | 'poll-invalid' | 'replace-votes-required' | 'poll-privacy-locked' | 'locked'
    /** Included on failure. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### अन्य नोट्स

- ऐसा `id` जो पोल पर नहीं है, या एक ही `id` दो बार देना, `poll-invalid` के साथ विफल होता है। बिना पोल वाले कमेंट के
  पास अभी कोई विकल्प id नहीं होता, इसलिए उसे भेजे गए हर विकल्प में `id` छोड़ना होगा।
- पोल की प्राइवेसी को संकुचित किया जा सकता है लेकिन वोट होने के बाद उसे बढ़ाया नहीं जा सकता।
- यह API आपकी साइट सेटिंग्स का पालन करता है। यदि साइट या पेज के लिए पोल सक्षम नहीं हैं, तो यह
  `polls-disabled` के साथ विफल होता है।
- लॉक किए गए कमेंट का पोल नहीं बदला जा सकता, और यह `locked` के साथ विफल होता है।
- कनेक्टेड विजेट लाइव अपडेट होते हैं, इसलिए दर्शक रीलोड किए बिना नया पोल देख सकते हैं।