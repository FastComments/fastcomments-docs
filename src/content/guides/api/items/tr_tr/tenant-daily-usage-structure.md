Bir `TenantDailyUsage` nesnesi, belirli bir gün için bir kiracının (tenant) kullanımını temsil eder. Belirli bir kiracı için belirli bir günde hiçbir etkinlik olmadıysa, o gün için bir `TenantDailyUsage` nesnesi bulunmaz.

`TenantDailyUsage` nesnesi gerçek zamanlı **değildir** ve gerçek kullanımın birkaç dakika gerisinde kalabilir.

`TenantDailyUsage` nesnesinin yapısı aşağıdaki gibidir:

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
    /** Faturalandırma için yok sayılır. **/
    ignored: boolean
}
[inline-code-end]