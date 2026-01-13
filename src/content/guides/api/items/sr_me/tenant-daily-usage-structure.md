Објекат `TenantDailyUsage` представља употребу за тeнанта за одређени дан. Ако није било активности за одређеног тeнанта на одређеном дану, тај дан неће имати `TenantDailyUsage` објекат.

Објекат `TenantDailyUsage` **није** у реалном времену и може заостајати неколико минута у односу на стварну употребу.

Структура објекта `TenantDailyUsage` је следећа:

[inline-code-attrs-start title = 'Структура објекта TenantDailyUsage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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