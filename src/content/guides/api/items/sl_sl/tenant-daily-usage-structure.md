Objekt `TenantDailyUsage` predstavlja porabo za najemnika na določen dan. Če ni bilo dejavnosti za določenega najemnika na določen dan, ta dan ne bo imel objekta `TenantDailyUsage`.

Objekt `TenantDailyUsage` **ni** v realnem času in je lahko za dejansko porabo zamaknjen za nekaj minut.

Struktura objekta `TenantDailyUsage` je naslednja:

[inline-code-attrs-start title = 'Struktura objekta TenantDailyUsage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** Prezrto pri obračunu. **/
    ignored: boolean
}
[inline-code-end]

---