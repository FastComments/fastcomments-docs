---
Bir `TenantDailyUsage` nesnesi, belirli bir gün için bir kiracının kullanımını temsil eder. Eğer belirli bir kiracı için belirli bir
günde hiçbir etkinlik yoksa, o gün için bir `TenantDailyUsage` nesnesi olmayacaktır.

The `TenantDailyUsage` object is **not** real time and may be minutes behind actual usage.

The structure for the `TenantDailyUsage` object is as follows:

[inline-code-attrs-start title = 'TenantDailyUsage Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** Faturalama için dikkate alınmaz. **/
    ignored: boolean
}
[inline-code-end]

---