वेबहुक के माध्यम से भेजी जाने वाली एकमात्र संरचना WebhookComment ऑब्जेक्ट है, जिसे नीचे TypeScript में दर्शाया गया है।

#### WebhookComment ऑब्जेक्ट की संरचना

##### "Create" इवेंट की संरचना
"create" इवेंट का रिक्वेस्ट बॉडी एक WebhookComment ऑब्जेक्ट होता है।

##### "Update" इवेंट की संरचना
"update" इवेंट का रिक्वेस्ट बॉडी एक WebhookComment ऑब्जेक्ट होता है।

##### "Delete" इवेंट की संरचना
"delete" इवेंट का रिक्वेस्ट बॉडी एक WebhookComment ऑब्जेक्ट होता है।

    14 नवंबर 2023 का परिवर्तन
    पहले "delete" इवेंट के रिक्वेस्ट बॉडी में केवल कमेंट की id होती थी। अब इसमें डिलीशन के समय का पूरा कमेंट होता है।


हर कुंजी (key) हमेशा बॉडी में मौजूद रहती है। जब किसी फ़ील्ड के लिए कमेंट में कोई मान नहीं होता, तो बॉडी में `null` होता है
(बूलियन के लिए `false` और लिस्ट के लिए `[]`), इसलिए डिलीवरी की संरचना एक कमेंट से दूसरे कमेंट में कभी नहीं बदलती।

[inline-code-attrs-start title = 'WebhookComment ऑब्जेक्ट'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** कमेंट की id। **/
    id: string
    /** कमेंट थ्रेड की पहचान करने वाली id या URL। सामान्यीकृत (Normalized)। **/
    urlId: string
    /** वह URL जो बताता है कि कमेंट कहाँ लिखा गया था। **/
    url: string | null
    /** कमेंट लिखने वाले यूज़र की id। SSO होने पर, tenant id के साथ प्रीफ़िक्स्ड। **/
    userId: string | null
    /** कमेंट लिखने वाले यूज़र का ईमेल। **/
    commenterEmail: string | null
    /** कमेंट विजेट में दिखने वाले यूज़र का नाम। SSO के साथ, यह displayName हो सकता है। **/
    commenterName: string
    /** रॉ (raw) कमेंट टेक्स्ट। **/
    comment: string
    /** पार्सिंग के बाद का कमेंट टेक्स्ट। **/
    commentHTML: string
    /** कमेंट की एक्सटर्नल id। **/
    externalId: string | null
    /** पैरेंट कमेंट की id। **/
    parentId: string | null
    /** वह UTC तारीख जब कमेंट लिखा गया था। **/
    date: UTC_ISO_DateString
    /** वोट्स का संयुक्त कर्मा (up - down)। **/
    votes: number
    votesUp: number
    votesDown: number
    /** True यदि कमेंट करते समय यूज़र लॉग इन था, या उसने कमेंट वेरिफ़ाई किया था, या कमेंट लिखते समय अपना सेशन वेरिफ़ाई किया था। **/
    verified: boolean
    /** वह UTC तारीख जब कमेंट वेरिफ़ाई किया गया था। **/
    verifiedDate: UTC_ISO_DateString | null
    /** यदि किसी मॉडरेटर ने कमेंट को रिव्यूड (reviewed) चिह्नित किया। **/
    reviewed: boolean
    /** अवतार (avatar) का लोकेशन, या base64 एनकोडिंग। केवल तब base64 होगा जब SSO के साथ यही मान भेजा गया हो। **/
    avatarSrc: string | null
    /** क्या कमेंट को मैन्युअली या स्वचालित रूप से स्पैम चिह्नित किया गया था? **/
    isSpam: boolean
    /** क्या कमेंट स्वचालित रूप से स्पैम चिह्नित किया गया था? **/
    aiDeterminedSpam: boolean
    /** क्या कमेंट में इमेज हैं? **/
    hasImages: boolean
    /** "Most Relevant" सॉर्ट दिशा के लिए कमेंट किस पेज नंबर पर है। **/
    pageNumber: number | null
    /** "Oldest First" सॉर्ट दिशा के लिए कमेंट किस पेज नंबर पर है। **/
    pageNumberOF: number | null
    /** "Newest First" सॉर्ट दिशा के लिए कमेंट किस पेज नंबर पर है। **/
    pageNumberNF: number | null
    /** क्या कमेंट स्वचालित रूप से या मैन्युअली अप्रूव किया गया था? **/
    approved: boolean
    /** कमेंट लिखते समय यूज़र का लोकेल कोड (फ़ॉर्मैट: en_us)। **/
    locale: string | null
    /** कमेंट में लिखे गए वे @mentions जो सफलतापूर्वक पार्स किए गए। कोई न होने पर खाली। **/
    mentions: CommentUserMention[]
    /** वह डोमेन जिससे कमेंट है। **/
    domain: string | null
    /** इस कमेंट से जुड़ी मॉडरेशन ग्रुप ids। कोई न होने पर खाली। **/
    moderationGroupIds: string[]
    /** इस कमेंट पर कमेंटर के नाम के आगे दिखाए गए बैज, इवेंट बनने के समय तक। कोई न होने पर खाली। **/
    badges: CommentUserBadgeInfo[]
}
[inline-code-end]

जब किसी कमेंट में यूज़र्स को टैग किया जाता है, तो जानकारी `mentions` नामक लिस्ट में संग्रहीत होती है। उस लिस्ट के प्रत्येक ऑब्जेक्ट की
निम्नलिखित संरचना होती है।

[inline-code-attrs-start title = 'Webhook Mentions ऑब्जेक्ट'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** यूज़र की id। SSO यूज़र्स के लिए, इसमें आपकी tenant id प्रीफ़िक्स्ड होगी। **/
    id: string
    /** अंतिम @mention टैग टेक्स्ट, @ प्रतीक सहित। **/
    tag: string
    /** मूल @mention टैग टेक्स्ट, @ प्रतीक सहित। **/
    rawTag: string
    /** किस प्रकार के यूज़र को टैग किया गया। user = FastComments.com अकाउंट। sso = SSOUser। **/
    type: 'user'|'sso'
    /** यदि यूज़र नोटिफ़िकेशन से ऑप्ट आउट करता है, तो भी यह true पर सेट रहेगा। **/
    sent: boolean
}
[inline-code-end]

`badges` लिस्ट की प्रत्येक एंट्री की निम्नलिखित संरचना होती है। बैज प्रकारों के लिए [Badges गाइड](/guide-badges.html) देखें।

[inline-code-attrs-start title = 'Webhook Badge ऑब्जेक्ट'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserBadgeInfo {
    /** इस यूज़र को दिए गए बैज की id। **/
    id: string
    /** बैज का प्रकार। **/
    type: number
    /** बैज का विवरण, बैज पर होवर करने पर दिखता है। **/
    description: string
    /** बैज पर दिखाया गया टेक्स्ट। **/
    displayLabel?: string | null
    /** बैज के लिए दिखाई गई इमेज, यदि वह एक इमेज का उपयोग करता है। **/
    displaySrc?: string | null
    backgroundColor?: string | null
    borderColor?: string | null
    textColor?: string | null
    /** बैज पर लागू एक अतिरिक्त CSS क्लास। **/
    cssClass?: string | null
}
[inline-code-end]

#### HTTP मेथड्स

आप एडमिन पैनल में प्रत्येक वेबहुक इवेंट प्रकार के लिए HTTP मेथड कॉन्फ़िगर कर सकते हैं:

- **Create इवेंट**: POST या PUT (डिफ़ॉल्ट: PUT)
- **Update इवेंट**: POST या PUT (डिफ़ॉल्ट: PUT)
- **Delete इवेंट**: DELETE, POST, या PUT (डिफ़ॉल्ट: DELETE)

चूंकि सभी रिक्वेस्ट में एक ID होती है, Create और Update ऑपरेशन डिफ़ॉल्ट रूप से इडेम्पोटेंट (idempotent) होते हैं (PUT)। उसी Create या Update रिक्वेस्ट को दोहराने से आपकी तरफ़ डुप्लिकेट ऑब्जेक्ट नहीं बनने चाहिए।

#### रिक्वेस्ट हेडर्स

प्रत्येक वेबहुक रिक्वेस्ट में निम्नलिखित हेडर्स शामिल होते हैं:

| हेडर | विवरण |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | आपकी API Secret |
| `X-FastComments-Timestamp` | Unix टाइमस्टैम्प (सेकंड) जब रिक्वेस्ट साइन किया गया था |
| `X-FastComments-Signature` | HMAC-SHA256 सिग्नेचर (`sha256=<hex>`) |

HMAC सिग्नेचर सत्यापित करने की जानकारी के लिए [Security & API Tokens](/guide-webhooks.html#webhooks-api-tokens) देखें।