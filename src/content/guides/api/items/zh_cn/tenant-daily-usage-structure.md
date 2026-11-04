`TenantDailyUsage` 对象表示某个租户在特定日期的使用情况。如果某个租户在某一天没有任何活动，则该天将不会有 `TenantDailyUsage` 对象。

`TenantDailyUsage` 对象**不是**实时的，可能比实际使用情况滞后几分钟。

`TenantDailyUsage` 对象的结构如下：

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
    /** 计费时忽略。 **/
    ignored: boolean
}
[inline-code-end]