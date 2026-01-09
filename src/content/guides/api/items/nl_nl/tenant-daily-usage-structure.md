Een `TenantDailyUsage` object represents the usage for a tenant on a given day. If there was no activity for a given tenant on a given
day, that day will not have a `TenantDailyUsage` object.

The `TenantDailyUsage` object is **not** real time and may be minutes behind actual usage.

The structure for the `TenantDailyUsage` object is as follows:

[inline-code-attrs-start title = 'Structuur van TenantDailyUsage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** Genegeerd voor facturering. **/
    ignored: boolean
}
[inline-code-end]

---