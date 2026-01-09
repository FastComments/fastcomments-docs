Објекат `TenantDailyUsage` представља коришћење за закупца на одређени дан. Ако није било активности за одређеног закупца на одређени
дан, тај дан неће имати `TenantDailyUsage` објекат.

Објекат `TenantDailyUsage` **није** у реалном времену и може заостајати неколико минута у односу на стварну употребу.

Структура за `TenantDailyUsage` објекат је следећа:

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