A `TenantDailyUsage` 物件代表租戶在特定日期的使用情況。如果在特定日期內該租戶沒有任何活動，則該日不會有 `TenantDailyUsage` 物件。

`TenantDailyUsage` 物件 **不是**即時的，可能會比實際使用量落後數分鐘。

`TenantDailyUsage` 物件的結構如下：

[inline-code-attrs-start title = 'TenantDailyUsage 結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** Ignored for billing. **/
    ignored: boolean
}
[inline-code-end]