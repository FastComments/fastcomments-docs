Objekt `TenantDailyUsage` predstavlja korištenje za tenant na određenom danu. Ako za određenog tenant-a na određen
dan nije bilo aktivnosti, taj dan neće imati objekt `TenantDailyUsage`.

Objekt `TenantDailyUsage` **nije** u stvarnom vremenu i može zaostajati nekoliko minuta za stvarnom upotrebom.

Struktura za objekt `TenantDailyUsage` je sljedeća:

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
    /** Ignorirano za naplatu. **/
    ignored: boolean
}
[inline-code-end]

---