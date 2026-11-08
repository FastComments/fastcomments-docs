`UserBadgeProgress` एक ऑब्जेक्ट है जो FastComments सिस्टम में विभिन्न बैज अर्जित करने की दिशा में उपयोगकर्ता की प्रगति को दर्शाता है।

यह ट्रैकिंग यह निर्धारित करने में मदद करती है कि उपयोगकर्ताओं को आपके समुदाय में उनकी गतिविधि और भागीदारी के आधार पर स्वचालित बैज कब प्राप्त होने चाहिए।

`UserBadgeProgress` ऑब्जेक्ट की संरचना इस प्रकार है:

[inline-code-attrs-start title = 'UserBadgeProgress संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadgeProgress {
    /** इस प्रगति रिकॉर्ड के लिए अद्वितीय पहचानकर्ता */
    id: string
    /** इस प्रगति रिकॉर्ड का संबंधित टेनेंट की ID */
    tenantId: string
    /** उस उपयोगकर्ता की ID जिसे यह प्रगति रिकॉर्ड ट्रैक करता है */
    userId: string
    /** सिस्टम में उपयोगकर्ता की पहली टिप्पणी की ID */
    firstCommentId?: string
    /** उपयोगकर्ता की पहली टिप्पणी की तिथि (epoch से मिलीसेकंड) */
    firstCommentDate?: number
    /** उपयोगकर्ता गतिविधि के आधार पर स्वचालित रूप से गणना किया गया ट्रस्ट फैक्टर */
    autoTrustFactor?: number
    /** व्यवस्थापकों द्वारा मैन्युअल रूप से सेट किया गया ट्रस्ट फैक्टर */
    manualTrustFactor?: number
    /** विभिन्न मेट्रिक्स के साथ विस्तृत प्रगति ऑब्जेक्ट, कुंजियाँ BadgeType enum से मेल खाती हैं */
    progress: {
        /** 0: CommentCount - उपयोगकर्ता द्वारा की गई टिप्पणियों की संख्या */
        '0'?: number
        /** 1: CommentUpVotes - उपयोगकर्ता को प्राप्त अपवोट की संख्या */
        '1'?: number
        /** 2: CommentReplies - उपयोगकर्ता द्वारा किए गए उत्तरों की संख्या */
        '2'?: number
        /** 3: CommentsPinned - उपयोगकर्ता की पिन की गई टिप्पणियों की संख्या */
        '3'?: number
        /** 4: Veteran - उपयोगकर्ता के खाते की आयु */
        '4'?: number
        /** 5: NightOwl - रात के समय उपयोगकर्ता द्वारा पोस्ट करने की बार-बारता */
        '5'?: number
        /** 6: FastReplyTime - मिलीसेकंड में औसत उत्तर समय */
        '6'?: number
        /** 7: ModeratorCommentsDeleted - मॉडरेटर बैज के लिए, हटाई गई टिप्पणियों की संख्या */
        '7'?: number
        /** 8: ModeratorCommentsApproved - मॉडरेटर बैज के लिए, स्वीकृत टिप्पणियों की संख्या */
        '8'?: number
        /** 9: ModeratorCommentsUnapproved - मॉडरेटर बैज के लिए, अस्वीकृत टिप्पणियों की संख्या */
        '9'?: number
        /** 10: ModeratorCommentsReviewed - मॉडरेटर बैज के लिए, समीक्षा की गई टिप्पणियों की संख्या */
        '10'?: number
        /** 11: ModeratorCommentsMarkedSpam - मॉडरेटर बैज के लिए, स्पैम के रूप में चिह्नित टिप्पणियों की संख्या */
        '11'?: number
        /** 12: ModeratorCommentsMarkedNotSpam - मॉडरेटर बैज के लिए, स्पैम नहीं के रूप में चिह्नित टिप्पणियों की संख्या */
        '12'?: number
        /** 13: RepliedToSpecificPage - प्रत्येक पेज के लिए, उत्तरों की संख्या */
        '13'?: Record<string, number>
    }
}
[inline-code-end]