[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments'; creditsCost = 1; api-resource-header-end]

यह API एंडपॉइंट कमेंट बनाने की क्षमता प्रदान करता है।

सामान्य उपयोग के मामले कस्टम UI, इंटीग्रेशन, या इंपोर्ट हैं।

नोट्स:

- यह API चाहें तो कमेंट विजेट को "live" अपडेट कर सकता है (इससे `creditsCost` `1` से `2` हो जाता है)।
- यह API ईमेल दिए जाने पर स्वतः हमारे सिस्टम में यूज़र ऑब्जेक्ट बना देगा।
- अलग-अलग ईमेल के साथ लेकिन एक ही यूज़रनेम वाले दो कमेंट सेव करने की कोशिश पर दूसरे कमेंट के लिए एरर आएगा।
- यदि आप `parentId` निर्दिष्ट कर रहे हैं, और चाइल्ड कमेंट का `notificationSentForParent` false है, तो **हम पैरेंट कमेंट के लिए नोटिफिकेशन भेजेंगे**। यह हर घंटे किया जाता है (हम भेजे जाने वाले ईमेल की संख्या कम करने के लिए नोटिफिकेशन को बैच में भेजते हैं)।
- यदि आप यूज़र बनाते समय वेलकम ईमेल, या कमेंट वेरिफिकेशन ईमेल भेजना चाहते हैं, तो क्वेरी पैरामीटर में `sendEmails` को `true` सेट करें।
- इस API के माध्यम से बनाए गए कमेंट एडमिन ऐप के Analytics और Moderation पेजों में दिखाई देंगे।
- यदि सेटिंग चालू है, तो "bad words" कमेंटर के नाम और कमेंट टेक्स्ट में अभी भी मास्क रहते हैं।
- इस API के माध्यम से बनाए गए कमेंट को चाहें तो स्पैम के लिए जांचा जा सकता है।
- अधिकतम कमेंट लंबाई जैसी कॉन्फ़िगरेशन, यदि Customization Rule एडमिन पेज के माध्यम से कॉन्फ़िगर की गई है, तो यहाँ लागू होगी।

सबमिट करने के लिए न्यूनतम डेटा जो कमेंट विजेट में दिखाई देगा, वह इस प्रकार है:

[inline-code-attrs-start title = 'न्यूनतम कमेंट POST cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"approved": true,
	"comment": "some-comment",
	"commenterName": "some-commenter-name",
	"date": 1622644382148,
	"urlId": "some-place",
	"url": "https://exmaple.com/some-page",
	"verified": true
}'
[inline-code-end]

एक अधिक वास्तविक रिक्वेस्ट इस तरह दिख सकती है:

[inline-code-attrs-start title = 'कमेंट POST cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true&doSpamCheck=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"approved": true,
	"avatarSrc": "https://static.fastcomments.com/1605337537848-DSC_0841.JPG",
	"comment": "some-comment",
	"commenterName": "some-commenter-name",
	"commenterEmail": "fordperfect@spaceship.com",
	"date": 1622644382148,
	"isSpam": false,
	"locale": "en_us",
	"notificationSentForParent": true,
	"notificationSentForParentTenant": true,
	"reviewed": true,
	"urlId": "some-place",
	"url": "https://exmaple.com/some-page",
	"verified": true,
	"votes": 1,
	"votesUp": 2,
	"votesDown": 1,
	"ip": "123.456.789.000"
}'
[inline-code-end]

[inline-code-attrs-start title = 'कमेंट POST रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPostQueryParams {
    tenantId: string
    API_KEY: string
    doSpamCheck?: 'true' | 'false'
	/** Whether the comment should appear "live" to users viewing instances of the comment widget with the same urlId. NOTE: Doubles credit cost from 1 to 2. **/
    isLive?: 'true' | 'false'
    sendEmails?: 'true' | 'false'
    populateNotifications?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'कमेंट POST रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email';
    /** Included on failure. **/
    reason?: string
    /** The created comment. **/
    comment?: Comment
    /** The associated user, which may or may not have already existed. **/
    user?: User
}
[inline-code-end]