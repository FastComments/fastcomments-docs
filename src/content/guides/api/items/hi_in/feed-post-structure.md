एक `FeedPost` ऑब्जेक्ट FastComments फ़ीड में एक पोस्ट को दर्शाता है। फ़ीड एक पोस्ट स्ट्रीम होती है जिसमें प्रत्येक पोस्ट की अपनी कमेंट थ्रेड होती है, और इसे Feed विजेट द्वारा रेंडर किया जाता है। हर पोस्ट का एक लेखक, वैकल्पिक रिच कंटेंट, मीडिया और लिंक होते हैं, और इसे टैग किया जा सकता है ताकि फ़ीड को फ़िल्टर किया जा सके।

`FeedPost` ऑब्जेक्ट की संरचना इस प्रकार है:

[inline-code-attrs-start title = 'FeedPost संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPost {
    /** READONLY **/
    _id: string
    /** READONLY **/
    tenantId: string
    title?: string
    /** पोस्ट के लेखक FastComments या SSO उपयोगकर्ता की id। **/
    fromUserId?: string
    /** सेट न होने पर उपयोगकर्ता से भरा जाता है। **/
    fromUserDisplayName?: string | null
    /** READONLY. उपयोगकर्ता से भरा जाता है। **/
    fromUserAvatar?: string | null
    /** फ़ीड को फ़िल्टर करने के लिए उपयोग किया जाता है। **/
    tags?: string[]
    /** फ़ीड के भीतर सॉर्ट वेट। उच्च मान पहले सॉर्ट होते हैं। **/
    weight?: number
    /** आपके अपने उपयोग के लिए फ्री-फॉर्म key/value जोड़े। **/
    meta?: Record<string, string>
    /** सैनिटाइज़ किया गया HTML। **/
    contentHTML?: string
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    /** READONLY **/
    createdAt: string
    /** READONLY. रिएक्शन प्रकार से गिनती। **/
    reacts?: Record<string, number>
    /** READONLY **/
    commentCount?: number | null
}

interface FeedPostMediaItem {
    title?: string
    /** क्लिक करने पर मीडिया आइटम कहाँ लिंक होता है। **/
    linkUrl?: string
    /** प्रत्येक रेंडिशन के लिए एक प्रविष्टि। विजेट सबसे उपयुक्त चुनता है। **/
    sizes: FeedPostMediaItemAsset[]
}

interface FeedPostMediaItemAsset {
    w: number
    h: number
    src: string
}

interface FeedPostLink {
    /** लिंक का टेक्स्ट, जैसे "Sign up now"। **/
    text?: string
    /** लिंक के साथ दिखाया जाने वाला शीर्षक। **/
    title?: string
    /** लिंक के साथ दिखाया जाने वाला विवरण। **/
    description?: string
    url?: string
}
[inline-code-end]

नोट्स:

- इनमें से कुछ फ़ील्ड `READONLY` चिह्नित हैं - ये API द्वारा लौटाए जाते हैं लेकिन सेट नहीं किए जा सकते।
- किसी पोस्ट पर कमेंट सामान्य कमेंट होते हैं जिनका `urlId` `post:` होता है जिसके बाद पोस्ट का `_id` आता है। किसी पोस्ट पर कमेंट पढ़ने या बनाने के लिए उस मान का उपयोग Comment API के साथ करें।