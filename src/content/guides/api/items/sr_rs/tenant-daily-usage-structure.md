Објекат `TenantDailyUsage` представља коришћење за тенанта на одређени дан. Ако за датог тенанта на одређени дан није било активности, тај дан неће имати објекат `TenantDailyUsage`.

Објекат `TenantDailyUsage` **није** у реалном времену и може заостајати неколико минута за стварним коришћењем.

Структура објекта `TenantDailyUsage` је следећа:

[inline-code-attrs-start title = 'Структура TenantDailyUsage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** Игнорисано за наплату. **/
    ignored: boolean
}
[inline-code-end]

---