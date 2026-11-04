Een `TenantDailyUsage` object vertegenwoordigt het gebruik voor een tenant op een bepaalde dag. Als er geen activiteit was voor een bepaalde tenant op een bepaalde
dag, zal die dag geen `TenantDailyUsage` object hebben.

Het `TenantDailyUsage` object is **niet** real-time en kan enkele minuten achterlopen op het werkelijke gebruik.

De structuur van het `TenantDailyUsage` object is als volgt:

[inline-code-attrs-start title = 'TenantDailyUsage Structuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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