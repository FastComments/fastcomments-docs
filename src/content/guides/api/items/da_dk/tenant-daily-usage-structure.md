Et `TenantDailyUsage`-objekt repræsenterer forbruget for en tenant på en given dag. Hvis der ikke var nogen aktivitet for en given tenant på en given
dag, vil den dag ikke have et `TenantDailyUsage`-objekt.

`TenantDailyUsage`-objektet er **ikke** realtid og kan være minutter efter det faktiske forbrug.

Strukturen for `TenantDailyUsage`-objektet er som følger:

[inline-code-attrs-start title = 'TenantDailyUsage Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
