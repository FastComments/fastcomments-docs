一个 `TenantDailyUsage` 对象表示租户在某一天的使用情况。如果在某租户的某个给定
天没有活动，则该天将不会有 `TenantDailyUsage` 对象。

The `TenantDailyUsage` object is **不是** real time and may be minutes behind actual usage.

The structure for the `TenantDailyUsage` object is as follows:

[inline-code-attrs-start title = 'TenantDailyUsage 结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** 忽略计费。 **/
    ignored: boolean
}
[inline-code-end]

---