एक `TenantDailyUsage` ऑब्जेक्ट किसी टेनेंट के लिए किसी दिए गए दिन के उपयोग को दर्शाता है। यदि किसी दिए गए दिन में किसी टेनेंट की कोई गतिविधि नहीं थी, तो उस दिन के लिए कोई `TenantDailyUsage` ऑब्जेक्ट नहीं होगा।

`TenantDailyUsage` ऑब्जेक्ट **रीयल टाइम नहीं** है और वास्तविक उपयोग से कुछ मिनट पीछे हो सकता है।

`TenantDailyUsage` ऑब्जेक्ट की संरचना इस प्रकार है:

[inline-code-attrs-start title = 'TenantDailyUsage संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface TenantDailyUsage {
    yearNumber: number
    monthNumber: number
    dayNumber: number
    commentFetchCount?: number
    commentCreateCount?: number
    conversationCreateCount?: number
    voteCount?: number
    accountCreatedCount?: number
    userMentionSearch?: number
    hashTagSearch?: number
    gifSearchTrending?: number
    gifSearch?: number
    apiCreditsUsed?: number
    createdAt: string
    billed: boolean
    /** बिलिंग के लिए अनदेखा। **/
    ignored: boolean
}
[inline-code-end]