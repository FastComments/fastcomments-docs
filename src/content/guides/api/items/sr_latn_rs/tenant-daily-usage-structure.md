Objekat `TenantDailyUsage` predstavlja upotrebu za tenant-a za određeni dan. Ako nije bilo aktivnosti za datog tenant-a na određen
dan, taj dan neće imati objekat `TenantDailyUsage`.

Objekat `TenantDailyUsage` je **NIJE** u realnom vremenu i može kasniti za nekoliko minuta u odnosu na stvarnu upotrebu.

Struktura objekta `TenantDailyUsage` je sledeća:

[inline-code-attrs-start title = 'Struktura TenantDailyUsage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** Ignorisano za naplatu. **/
    ignored: boolean
}
[inline-code-end]

---