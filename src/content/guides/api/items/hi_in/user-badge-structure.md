`UserBadge` एक ऑब्जेक्ट है जो FastComments सिस्टम में किसी उपयोगकर्ता को सौंपे गए बैज को दर्शाता है।

बैज उपयोगकर्ताओं को उनकी गतिविधि (जैसे कमेंट संख्या, रिप्लाई समय, वेटेरन स्थिति) के आधार पर स्वचालित रूप से या साइट व्यवस्थापकों द्वारा मैन्युअल रूप से सौंपे जा सकते हैं।

`UserBadge` ऑब्जेक्ट की संरचना इस प्रकार है:

[inline-code-attrs-start title = 'UserBadge संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadge {
    /** इस उपयोगकर्ता बैज असाइनमेंट के लिए अद्वितीय पहचानकर्ता */
    id: string
    /** वह उपयोगकर्ता की ID जिसे यह बैज सौंपा गया है */
    userId: string
    /** टेनेंट के बैज कैटलॉग से बैज परिभाषा की ID */
    badgeId: string
    /** वह टेनेंट की ID जिसने यह बैज बनाया/सौंपा */
    fromTenantId: string
    /** यह बैज कब बनाया गया (epoch से मिलीसेकंड) */
    createdAt?: number
    /** उपयोगकर्ता को यह बैज कब प्राप्त हुआ (epoch से मिलीसेकंड) */
    receivedAt?: number
    /** 
     * बैज का प्रकार: 
     * 0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, 3=CommentsPinned, 
     * 4=Veteran, 5=NightOwl, 6=FastReplyTime, 7=ModeratorCommentsDeleted,
     * 8=ModeratorCommentsApproved, 9=ModeratorCommentsUnapproved, 10=ModeratorCommentsReviewed,
     * 11=ModeratorCommentsMarkedSpam, 12=ModeratorCommentsMarkedNotSpam, 13=RepliedToSpecificPage,
     * 14=Manual
     */
    type: number
    /** थ्रेशोल्ड-आधारित बैज के लिए, थ्रेशोल्ड मान */
    threshold?: number
    /** बैज का नाम/लेबल */
    name?: string
    /** बैज का विस्तृत विवरण */
    description?: string
    /** बैज पर दिखाया गया टेक्स्ट */
    displayLabel?: string
    /** बैज पर दिखाई जाने वाली छवि का URL */
    displaySrc?: string
    /** बैज के लिए बैकग्राउंड रंग (हेक्स कोड) */
    backgroundColor?: string
    /** बैज के लिए बॉर्डर रंग (हेक्स कोड) */
    borderColor?: string
    /** बैज के लिए टेक्स्ट रंग (हेक्स कोड) */
    textColor?: string
    /** स्टाइलिंग के लिए अतिरिक्त CSS क्लास */
    cssClass?: string
    /** वेटेरन बैज के लिए, मिलीसेकंड में समय थ्रेशोल्ड */
    veteranUserThresholdMillis?: number
    /** क्या यह बैज उपयोगकर्ता के कमेंट्स पर दिखाया जाता है */
    displayedOnComments: boolean
    /** बैज का प्रदर्शन क्रम */
    order?: number
    /** यदि सेट है, तो यह बैज केवल मेल खाते urlId वाले पेज पर दिखाया जाता है। ग्लोबल बैज के लिए Null। */
    urlId?: string | null
}
[inline-code-end]