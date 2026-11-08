एक `Notification` ऑब्जेक्ट किसी उपयोगकर्ता के लिए एक नोटिफिकेशन को दर्शाता है।

`Notification` ऑब्जेक्ट्स स्वचालित रूप से बनाए जाते हैं और इन्हें API के माध्यम से नहीं बनाया जा सकता। ये एक वर्ष बाद समाप्त (expire) भी हो जाते हैं।
नोटिफिकेशन को डिलीट नहीं किया जा सकता। हालाँकि, इन्हें अपडेट करके `viewed` को `false` सेट किया जा सकता है, और आप `viewed` के आधार पर क्वेरी कर सकते हैं।

एक उपयोगकर्ता किसी विशेष कमेंट के लिए नोटिफिकेशन से ऑप्ट आउट भी कर सकता है, नोटिफिकेशन में `optedOut` को `true` सेट करके। आप इसे `false` सेट करके दोबारा ऑप्ट इन कर सकते हैं।

विभिन्न नोटिफिकेशन प्रकार होते हैं - `relatedObjectType` और `type` देखें।

नोटिफिकेशन बनाने के तरीके काफी लचीले हैं और इन्हें कई परिदृश्यों द्वारा ट्रिगर किया जा सकता है (`NotificationType` देखें)।

आज की तारीख तक, एक `Notification` का अस्तित्व वास्तव में यह नहीं दर्शाता कि कोई ईमेल भेजा गया है या भेजा जाना चाहिए। बल्कि, नोटिफिकेशन का उपयोग नोटिफिकेशन फ़ीड और संबंधित इंटीग्रेशन के लिए किया जाता है।

`Notification` ऑब्जेक्ट की संरचना इस प्रकार है:

[inline-code-attrs-start title = 'Notification संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum NotificationObjectType {
    Comment = 0,
    Profile = 1,
    Tenant = 2
}

enum NotificationType {
    /** अगर किसी ने आपको रिप्लाई किया। **/
    RepliedToMe = 0,
    /** अगर किसी ने ऐसी थ्रेड में कहीं भी रिप्लाई किया (बच्चों के बच्चों सहित) जिस थ्रेड पर आपने कमेंट किया था। **/
    RepliedTransientChild = 1,
    /** अगर आपके कमेंट को अप-वोट किया गया। **/
    VotedMyComment = 2,
    /** अगर आपकी सदस्यता वाले पेज के रूट पर नया कमेंट किया गया। **/
    SubscriptionReplyRoot = 3,
    /** अगर किसी ने आपकी प्रोफ़ाइल पर कमेंट किया। **/
    CommentedOnProfile = 4,
    /** अगर आपके पास कोई DM है। **/
    DirectMessage = 5,
    /** TrialLimits केवल tenant उपयोगकर्ताओं के लिए है। **/
    TrialLimits = 6,
    /** अगर आपको @mentioned किया गया। **/
    Mentioned = 7
}

interface Notification {
    id: string
    tenantId: string
    /** SSO के साथ, user id का फ़ॉर्मैट `<tenant id>:<user id>` होता है। **/
    userId?: string
    /** SSO के साथ काम करते समय, आपको केवल `userId` की चिंता करनी होती है। **/
    anonUserId?: string
    /** urlId लगभग हमेशा परिभाषित होता है। यह केवल tenant-स्तरीय नोटिफिकेशन के लिए वैकल्पिक होता है, जो दुर्लभ होते हैं। **/
    urlId?: string
    /** नोटिफिकेशन के स्रोत तक तेज़ी से नेविगेट करने के लिए URL कैश किया जाता है। **/
    url?: string
    /** नोटिफिकेशन स्रोत को तेज़ी से पढ़ने के लिए Page Title कैश किया जाता है। **/
    pageTitle?: string
    relatedObjectType: NotificationObjectType
    /** उदाहरण के लिए, comment id। **/
    relatedObjectId: string
    viewed: boolean
    createdAt: string // date string
    type: NotificationType
    fromCommentId?: string
    fromVoteId?: string
    /** नोटिफिकेशन को तेज़ी से दिखाने के लिए fromUserName और fromUserAvatarSrc यहाँ कैश किए जाते हैं। जब उपयोगकर्ता अपडेट होता है तो ये अपडेट हो जाते हैं। **/
    fromUserName: string
    fromUserId: string
    fromUserAvatarSrc?: string
    /** इस ऑब्जेक्ट के लिए नोटिफिकेशन पाना बंद करने के लिए इसे true सेट करें। **/
    optedOut?: boolean
}
[inline-code-end]