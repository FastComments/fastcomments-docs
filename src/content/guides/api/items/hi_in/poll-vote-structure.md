एक `PollVote` किसी पोल पर एक व्यक्ति का उत्तर होता है। पोल पर दिखाए जाने वाले काउंट इनके साथ सिंक में रखे जाते हैं,
इसलिए आपको ये केवल तब चाहिए जब आप यह जानना चाहें कि *किसने* किस विकल्प के लिए वोट दिया, न कि कुल संख्या।

एक वोटर का प्रति पोल अधिकतम एक वोट होता है। दोबारा वोट देने पर दूसरा वोट जोड़ने के बजाय उनका मौजूदा वोट नए विकल्प पर चला जाता है,
और `updatedAt` यह रिकॉर्ड करता है कि वह कब हुआ।

`voterId` वह `userId` होता है जब वोटर लॉग इन था, अन्यथा वह `anonUserId` होता है।

[inline-code-attrs-start title = 'PollVote संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVote {
    id: string
    tenantId: string
    commentId: string
    urlId: string
    /** वोटर लॉग इन होने पर userId, अन्यथा anonUserId। **/
    voterId: string
    optionId: string
    createdAt: string
    /** जब वोटर ने अंतिम बार अपना वोट किसी अन्य विकल्प पर स्थानांतरित किया। **/
    updatedAt?: string
}
[inline-code-end]

### गोपनीयता

पोल की `privacy` सेटिंग इस API पर उसी तरह लागू होती है जिस तरह वह कमेंट विजेट में लागू होती है:

- **Anonymous** (डिफ़ॉल्ट): कोई भी नहीं देख सकता कि किसी ने कैसे वोट दिया, इसलिए वोट पढ़े नहीं जा सकते।
  `GET /api/v1/poll-votes` और `GET /api/v1/poll-votes/:id` `poll-anonymous` के साथ प्रतिक्रिया देते हैं। पोल के काउंट
  अभी भी `GET /api/v1/polls/:commentId` से उपलब्ध हैं।
- **Admins और moderators**: आपकी API key आपकी साइट के एडमिन की होती है, इसलिए यह वोट पढ़ सकती है।
- **Everyone**: वोट पढ़े जा सकते हैं।

पोल की गोपनीयता वोट होने के बाद संकुचित की जा सकती है लेकिन बढ़ाई नहीं जा सकती।