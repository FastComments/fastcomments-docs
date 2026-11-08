एक `Comment` ऑब्जेक्ट किसी उपयोगकर्ता द्वारा छोड़ी गई टिप्पणी को दर्शाता है।

पैरेंट और चाइल्ड कमेंट्स के बीच संबंध `parentId` के माध्यम से परिभाषित होता है।

Comment ऑब्जेक्ट की संरचना इस प्रकार है:

[inline-code-attrs-start title = 'Comment संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Comment {
    /** READONLY: यदि स्पैम इंजन ने कमेंट को स्पैम निर्धारित किया तो true पर सेट होता है। **/
    aiDeterminedSpam?: boolean
    /** क्या कमेंट दिखाने के लिए स्वीकृत है। कमेंट सेव करते समय true पर सेट करें, अन्यथा यह छिपा रहेगा। **/
    approved?: boolean
    /** उपयोगकर्ता का अवतार। **/
    avatarSrc?: string
    /** चाइल्ड कमेंट्स। सभी परिदृश्यों में पॉपुलेटेड नहीं होते। API के माध्यम से asTree true पर सेट होने पर उपयोग होता है। **/
    children: Comment[]
    /** कमेंटर की रॉ कमेंट। **/
    comment: string
    /** READONLY: कमेंटर की कमेंट HTML में पार्स की गई। **/
    commentHTML?: string
    /** कमेंटर का ईमेल। यदि अनाम कमेंटिंग बंद है तो आवश्यक। **/
    commenterEmail?: string
    /** कमेंटर का लिंक (उदाहरण के लिए, उनका ब्लॉग)। **/
    commenterLink?: string
    /** कमेंटर का नाम। हमेशा आवश्यक। यदि उपलब्ध नहीं है, तो "Anonymous" जैसा कुछ सेट करें। **/
    commenterName: string
    /** कमेंट छोड़ी गई तिथि, UTC epoch में। **/
    date: number
    /** कमेंट के लिए "डिस्प्ले लेबल" - उदाहरण के लिए "Admin", "Moderator", या "VIP User" जैसा कुछ। **/
    displayLabel?: string
    /** वह डोमेन जहाँ कमेंट पोस्ट की गई। **/
    domain?: string
    /** READONLY: कमेंट कितनी बार फ्लैग की गई। **/
    flagCount?: number
    /** कमेंट में लिखे गए #hashtags जो सफलतापूर्वक पार्स हुए। आप क्वेरी के लिए मैन्युअल रूप से भी हैशटैग जोड़ सकते हैं, लेकिन वे कमेंट टेक्स्ट में स्वतः डिस्प्ले नहीं होंगे। **/
    hashTags?: CommentHashTag[]
    /** READONLY: क्या कमेंट में इमेज हैं? **/
    hasImages?: boolean
    /** READONLY: क्या कमेंट में लिंक हैं? **/
    hasLinks?: boolean
    /** READONLY: अद्वितीय कमेंट आईडी। **/
    id: string
    /** केवल बनाते समय! इसे स्टोरेज के लिए हैश किया जाता है। **/
    ip?: string
    /** READONLY: क्या वर्तमान उपयोगकर्ता ने इस कमेंट को लिखने वाले उपयोगकर्ता को ब्लॉक किया है? **/
    isBlocked?: boolean
    /** READONLY: क्या कमेंट एडमिन द्वारा है? userId के आधार पर स्वतः सेट होता है। **/
    isByAdmin?: boolean
    /** READONLY: क्या कमेंट मॉडरेटर द्वारा है? userId के आधार पर स्वतः सेट होता है। **/
    isByModerator?: boolean
    /** यदि कमेंट सॉफ्ट डिलीट की गई थी तो true पर सेट (कुछ अन्य कॉन्फ़िगरेशन के कारण प्लेसहोल्डर छोड़ना पड़ा)। **/
    isDeleted?: boolean
    /** यदि उपयोगकर्ता का खाता डिलीट हो गया था और कमेंट को बनाए रखना पड़ा तो true पर सेट। **/
    isDeletedUser?: boolean
    /** READONLY: क्या वर्तमान में लॉग-इन उपयोगकर्ता (contextUserId) द्वारा फ्लैग की गई है? **/
    isFlagged?: boolean
    /** क्या कमेंट पिन की गई है? **/
    isPinned?: boolean
    /** क्या कमेंट लॉक है? true होने पर, अनलॉक होने तक कोई भी (मॉडरेटर सहित) उत्तर, संपादन या डिलीट नहीं कर सकता। **/
    isLocked?: boolean
    /** क्या कमेंट स्पैम है? **/
    isSpam?: boolean
    /** READONLY: क्या वर्तमान उपयोगकर्ता (contextUserId) के लिए कमेंट डाउनवोट की गई है? **/
    isVotedDown?: boolean
    /** READONLY: क्या वर्तमान उपयोगकर्ता (contextUserId) के लिए कमेंट अपवोट की गई है? **/
    isVotedUp?: boolean
    /** कमेंट की लोकेल। यदि प्रदान नहीं की गई, तो language accept HTTP हेडर से प्राप्त की जाएगी। **/
    locale?: 'de_de' | 'en_us' | 'es_es' | 'fr_fr' | 'it_it' | 'ja_jp' | 'ko_kr' | 'pl_pl' | 'pt_br' | 'ru_ru' | 'tr_tr' | 'zh_cn' | 'zh_tw'
    /** READONLY: कमेंट में लिखे गए @mentions जो सफलतापूर्वक पार्स हुए। **/
    mentions?: CommentUserMention[]
    /** कमेंट से जुड़ा वैकल्पिक मेटाडेटा। **/
    meta?: Record<string, string | number | boolean>
    /** इस कमेंट से जुड़े मॉडरेशन ग्रुप आईडी की वैकल्पिक सूची। **/
    moderationGroupIds?: string[]|null
    /** READONLY: इस कमेंट पर वर्तमान उपयोगकर्ता (contextUserId) के वोट से संबंधित वोट ऑब्जेक्ट की आईडी। **/
    myVoteId?: string
    /** क्या इस कमेंट के लिए कमेंटर्स को सूचनाएं भेजी गईं। इंपोर्ट पर सूचनाएं भेजने से रोकने के लिए, इसे true पर सेट करें। **/
    notificationSentForParent?: boolean
    /** क्या इस कमेंट के लिए टेनेंट उपयोगकर्ताओं को सूचनाएं भेजी गईं। इंपोर्ट पर सूचनाएं भेजने से रोकने के लिए, इसे true पर सेट करें। **/
    notificationSentForParentTenant?: boolean
    /** वह पेज शीर्षक जिस पर यह कमेंट थी। **/
    pageTitle?: string
    /** यदि हम किसी कमेंट का उत्तर दे रहे हैं, तो यह उस आईडी है जिसका उत्तर दिया जा रहा है। **/
    parentId?: string|null
    /** क्या कमेंट समीक्षित (reviewed) चिह्नित है। **/
    reviewed: boolean
    /** वह टेनेंट आईडी जिससे कमेंट संबंधित है। **/
    tenantId: string
    /** वह उपयोगकर्ता जिसने कमेंट लिखी। नाम/ईमेल के साथ कमेंट सेव करते समय स्वतः बनाया जाता है। **/
    userId?: string|null
    /** वह URL जहाँ यह कमेंट दृश्यमान है, जैसे ब्लॉग पोस्ट। **/
    url: string
    /** आपके द्वारा पास किए गए urlId का "साफ़" संस्करण। सेव करते समय, आप यह फ़ील्ड निर्दिष्ट करते हैं, लेकिन कमेंट वापस फ़ेच करने पर यह "साफ़" होगा और आपका मूल मान "urlIdRaw" में चला जाएगा। **/
    urlId: string
    /** READONLY: आपके द्वारा पास किया गया मूल urlId। **/
    urlIdRaw?: string
    /** क्या उपयोगकर्ता और यह कमेंट सत्यापित हैं? **/
    verified: boolean
    /** अपवोट की संख्या। **/
    votesUp?: number
    /** डाउनवोट की संख्या। **/
    votesDown?: number
    /** कमेंट का "कर्मा" (= अपवोट - डाउनवोट)। **/
    votes?: number
}
[inline-code-end]

इनमें से कुछ फ़ील्ड `READONLY` चिह्नित हैं - ये API द्वारा लौटाई जाती हैं लेकिन सेट नहीं की जा सकतीं।

### कमेंट टेक्स्ट संरचना

कमेंट्स FastComments के मार्कडाउन फ्लेवर में लिखी जाती हैं, जो मार्कडाउन के साथ इमेज के लिए पारंपरिक `bbcode` शैली के टैग जैसे `[img]path[/img]` हैं।

टेक्स्ट दो फ़ील्ड में संग्रहीत होता है। उपयोगकर्ता द्वारा दर्ज किया गया टेक्स्ट बिना बदले `comment` फ़ील्ड में संग्रहीत होता है। यह रेंडर होकर `commentHTML` फ़ील्ड में संग्रहीत होता है।

अनुमत HTML टैग हैं: `b, u, i, strike, pre, span, code, img, a, strong, ul, ol, li, और br`।

HTML रेंडर करने की सिफारिश की जाती है, क्योंकि यह HTML का बहुत छोटा सबसेट है, रेंडरर बनाना काफी सरल है। उदाहरण के लिए, React Native और Flutter के लिए इसमें मदद के लिए कई लाइब्रेरीज़ उपलब्ध हैं।

आप `comment` फ़ील्ड का अन-नॉर्मलाइज़्ड मान रेंडर करना चुन सकते हैं। [एक उदाहरण पार्सर यहाँ है।](https://github.com/FastComments/fastcomments-code-examples/blob/master/custom-client/client/parse-comment.js)।

उदाहरण पार्सर को HTML के साथ काम करने के लिए भी समायोजित किया जा सकता है, और HTML टैग को आपके प्लेटफ़ॉर्म के लिए रेंडर करने योग्य अपेक्षित तत्वों में बदला जा सकता है।

### टैगिंग

जब कमेंट में उपयोगकर्ताओं को टैग किया जाता है, तो जानकारी `mentions` नामक सूची में संग्रहीत होती है। उस सूची में प्रत्येक ऑब्जेक्ट की संरचना निम्नलिखित है।

[inline-code-attrs-start title = 'Comment Mentions ऑब्जेक्ट'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** उपयोगकर्ता आईडी। SSO उपयोगकर्ताओं के लिए, इसमें आपकी टेनेंट आईडी प्रीफ़िक्स्ड होगी। **/
    id: string
    /** अंतिम @mention टैग टेक्स्ट, @ प्रतीक सहित। **/
    tag: string
    /** मूल @mention टैग टेक्स्ट, @ प्रतीक सहित। **/
    rawTag: string
    /** किस प्रकार के उपयोगकर्ता को टैग किया गया। user = FastComments.com खाता। sso = SSOUser। **/
    type: 'user'|'sso'
    /** यदि उपयोगकर्ता सूचनाओं से ऑप्ट आउट करता है, तो भी यह true पर सेट रहेगा। **/
    sent: boolean
}
[inline-code-end]

### हैशटैग

जब हैशटैग का उपयोग किया जाता है और सफलतापूर्वक पार्स किया जाता है, तो जानकारी `hashTags` नामक सूची में संग्रहीत होती है। उस सूची में प्रत्येक ऑब्जेक्ट की संरचना निम्नलिखित है। यदि `retain` सेट है, तो क्वेरी के लिए हैशटैग को मैन्युअल रूप से कमेंट के `hashTags` ऐरे में भी जोड़ा जा सकता है।

[inline-code-attrs-start title = 'Comment HashTag ऑब्जेक्ट'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentHashTag {
    /** हैशटैग आईडी। **/
    id: string
    /** अंतिम #hashtag टैग टेक्स्ट, # प्रतीक सहित। **/
    tag: string
    /** यदि हैशटैग कस्टम URL से जुड़ा है, तो यह परिभाषित होगा। **/
    url?: string
    /** यदि कमेंट अपडेट होने पर हैशटैग को बनाए रखना चाहिए, भले ही वह कमेंट टेक्स्ट में मौजूद न हो। कमेंट टेक्स्ट बदले बिना कमेंट्स को टैग करने के लिए उपयोगी। **/
    retain?: boolean
}
[inline-code-end]