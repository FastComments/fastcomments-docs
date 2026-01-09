Объект `TenantDailyUsage` представляет использование для арендатора за указанный день. Если для данного арендатора в указанный
день не было активности, у этого дня не будет объекта `TenantDailyUsage`.

Объект `TenantDailyUsage` **не** является данными в реальном времени и может отставать от фактического использования на несколько минут.

Структура объекта `TenantDailyUsage` выглядит следующим образом:

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
    /** Игнорируется для выставления счетов. **/
    ignored: boolean
}
[inline-code-end]

---