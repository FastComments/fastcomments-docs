Obiekt `TenantDailyUsage` reprezentuje użycie dla najemcy w danym dniu. Jeżeli nie było aktywności dla danego najemcy na danym
dniu, ten dzień nie będzie miał obiektu `TenantDailyUsage`.

Obiekt `TenantDailyUsage` **nie** jest w czasie rzeczywistym i może być opóźniony o kilka minut względem rzeczywistego użycia.

Struktura obiektu `TenantDailyUsage` jest następująca:

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
    /** Pomijane przy rozliczaniu. **/
    ignored: boolean
}
[inline-code-end]

---