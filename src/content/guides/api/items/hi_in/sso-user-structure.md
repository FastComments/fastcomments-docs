FastComments एक उपयोग में आसान SSO समाधान प्रदान करता है। HMAC-आधारित इंटीग्रेशन के साथ उपयोगकर्ता की जानकारी अपडेट करना उतना ही सरल है जितना उपयोगकर्ता को अपडेटेड पेलोड के साथ पेज लोड करवाना।

हालाँकि, अपने एप्लिकेशन की निरंतरता (consistency) सुधारने के लिए उस फ्लो के बाहर उपयोगकर्ता को प्रबंधित करना वांछनीय हो सकता है।

SSO User API उन ऑब्जेक्ट्स को CRUD करने का एक तरीका प्रदान करती है जिन्हें हम SSOUsers कहते हैं। ये ऑब्जेक्ट्स सामान्य Users से अलग होते हैं और टाइप सेफ्टी के लिए अलग रखे जाते हैं।

SSOUser ऑब्जेक्ट की संरचना इस प्रकार है:

[inline-code-attrs-start title = 'SSOUser संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    id: string
    username: string
    email?: string
    websiteUrl?: string
    signUpDate: number
    createdFromUrlId?: string
    loginCount?: number
    avatarSrc?: string
    optedInNotifications?: boolean
    optedInSubscriptionNotifications?: boolean
    displayLabel?: string
    displayName?: string
    isAccountOwner?: boolean // Admin permission - SSO users with this flag are billed as SSO Admins (separate from regular SSO users)
    isAdminAdmin?: boolean // Admin permission - SSO users with this flag are billed as SSO Admins (separate from regular SSO users)
    isCommentModeratorAdmin?: boolean // Moderator permission - SSO users with this flag are billed as SSO Moderators (separate from regular SSO users)
    /** If null, Access Control will not be applied to the user. If an empty list, this user will not be able to see any pages or @mention other users. **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** Don't let other users see this user's activity, including comments, on their profile. Default is true to provide secure profiles by default. **/
    isProfileActivityPrivate?: boolean
    /** Don't let other users leave comments on the user's profile, or see existing profile comments. Default false. **/
    isProfileCommentsPrivate?: boolean
    /** Don't let other users send direct messages to this user. Default false. **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** Optional configuration for user badges. **/
    badgeConfig?: {
        /** Array of badge IDs to assign to the user. Limited to 30 badges. Order is respected. These are global badges visible on all pages. **/
        badgeIds: string[]
        /** Array of badge IDs scoped to the current page (urlId). These badges are only displayed on the page where they were assigned. **/
        pageBadgeIds?: string[]
        /** If true, replaces all existing displayed badges with the provided ones. Global and page-scoped badges are overridden independently. If false, adds to existing badges. **/
        override?: boolean
        /** If true, updates badge display properties from tenant configuration. **/
        update?: boolean
    }
}
[inline-code-end]

### SSO उपयोगकर्ताओं के लिए बिलिंग

SSO उपयोगकर्ताओं की बिलिंग उनके परमिशन फ्लैग्स के आधार पर अलग-अलग होती है:

- **सामान्य SSO उपयोगकर्ता**: बिना एडमिन या मॉडरेटर परमिशन वाले उपयोगकर्ता सामान्य SSO उपयोगकर्ताओं के रूप में बिल किए जाते हैं
- **SSO एडमिन**: `isAccountOwner` या `isAdminAdmin` फ्लैग्स वाले उपयोगकर्ता SSO एडमिन के रूप में अलग से बिल किए जाते हैं (सामान्य टेनेंट एडमिन के समान दर पर)
- **SSO मॉडरेटर**: `isCommentModeratorAdmin` फ्लैग वाले उपयोगकर्ता SSO मॉडरेटर के रूप में अलग से बिल किए जाते हैं (सामान्य मॉडरेटर के समान दर पर)

**महत्वपूर्ण**: दोहरी बिलिंग रोकने के लिए, सिस्टम ईमेल पते के आधार पर SSO उपयोगकर्ताओं को सामान्य टेनेंट उपयोगकर्ताओं और मॉडरेटर्स के विरुद्ध स्वचालित रूप से डीडुप्लिकेट करता है। यदि किसी SSO उपयोगकर्ता का ईमेल किसी सामान्य टेनेंट उपयोगकर्ता या मॉडरेटर के समान है, तो उनसे दो बार बिल नहीं किया जाएगा।

### एक्सेस कंट्रोल

उपयोगकर्ताओं को समूहों में विभाजित किया जा सकता है। `groupIds` फ़ील्ड इसी काम के लिए है, और यह वैकल्पिक (optional) है।

### @Mentions

डिफ़ॉल्ट रूप से, जब `@` कैरेक्टर टाइप किया जाता है तो `@mentions` अन्य SSO उपयोगकर्ताओं को खोजने के लिए `username` का उपयोग करेगा। यदि `displayName` का उपयोग किया जाता है, तो जब `displayName` के लिए मैच होता है, `username` से मैच होने वाले परिणामों को अनदेखा कर दिया जाएगा, और `@mention` खोज परिणाम `displayName` का उपयोग करेंगे।

### सब्सक्रिप्शन

FastComments के साथ, उपयोगकर्ता कमेंट विजेट में घंटी आइकन पर क्लिक करके और Subscribe पर क्लिक करके किसी पेज की सदस्यता ले सकते हैं।

सामान्य उपयोगकर्ता के साथ, हम उन्हें उनकी नोटिफिकेशन सेटिंग्स के आधार पर नोटिफिकेशन ईमेल भेजते हैं।

SSO उपयोगकर्ताओं के साथ, हम इसे पिछली संगतता (backwards compatibility) के लिए विभाजित करते हैं। उपयोगकर्ताओं को ये अतिरिक्त सब्सक्रिप्शन नोटिफिकेशन ईमेल तभी भेजे जाएंगे जब आप `optedInSubscriptionNotifications` को `true` पर सेट करें।

### बैज

आप `badgeConfig` प्रॉपर्टी का उपयोग करके SSO उपयोगकर्ताओं को बैज असाइन कर सकते हैं। बैज विज़ुअल इंडिकेटर होते हैं जो कमेंट्स में उपयोगकर्ता के नाम के बगल में दिखाई देते हैं।

- `badgeIds` - उपयोगकर्ता को असाइन करने के लिए बैज ID की एक सरणी। ये सभी पेजों पर दिखाई देने वाले ग्लोबल बैज हैं। आपके FastComments खाते में बनाई गई वैध बैज ID होनी चाहिए। अधिकतम 30 बैज।
- `pageBadgeIds` - वर्तमान पेज (`urlId`) तक सीमित बैज ID की एक वैकल्पिक सरणी। ये बैज केवल उस पेज पर प्रदर्शित होते हैं जहाँ उन्हें असाइन किया गया था। एक ही उपयोगकर्ता के लिए अलग-अलग पेजों पर अलग-अलग पेज-स्कोप्ड बैज हो सकते हैं।
- `override` - यदि true है, तो सभी मौजूदा प्रदर्शित बैज प्रदान किए गए बैजों से बदल दिए जाएंगे। ग्लोबल और पेज-स्कोप्ड बैज स्वतंत्र रूप से ओवरराइड किए जाते हैं — ग्लोबल बैज को ओवरराइड करना पेज-स्कोप्ड बैज को प्रभावित नहीं करता, और इसके विपरीत भी। यदि false या छोड़ा गया हो, तो प्रदान किए गए बैज किसी भी मौजूदा बैज में जोड़ दिए जाएंगे।
- `update` - यदि true है, तो जब भी उपयोगकर्ता लॉग इन करेगा, बैज डिस्प्ले प्रॉपर्टीज़ टेनेंट कॉन्फ़िगरेशन से अपडेट कर दी जाएंगी।