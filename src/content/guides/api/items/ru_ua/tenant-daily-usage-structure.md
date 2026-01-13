Объект `TenantDailyUsage` представляет собой данные об использовании для тенанта за конкретный день. Если для данного тенанта в определённый
день, у этого дня не будет объекта `TenantDailyUsage`.

Объект `TenantDailyUsage` **не** является данными в режиме реального времени и может отставать на несколько минут от фактического использования.

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
    /** Игнорируется при выставлении счетов. **/
    ignored: boolean
}
[inline-code-end]

---