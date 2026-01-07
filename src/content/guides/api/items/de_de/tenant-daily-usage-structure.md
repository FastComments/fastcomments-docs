Ein `TenantDailyUsage`-Objekt repräsentiert die Nutzung für einen Tenant an einem bestimmten Tag. Wenn es keine Aktivität für einen bestimmten Tenant an einem bestimmten
Tag gab, wird dieser Tag kein `TenantDailyUsage`-Objekt haben.

Das `TenantDailyUsage`-Objekt ist **nicht** in Echtzeit und kann einige Minuten hinter der tatsächlichen Nutzung zurückliegen.

Die Struktur des `TenantDailyUsage`-Objekts ist wie folgt:

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
