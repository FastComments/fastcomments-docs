Об'єкт `TenantDailyUsage` представляє використання для орендаря за конкретний день. Якщо для певного орендаря в певний
день не було активності, у цей день не буде об'єкта `TenantDailyUsage`.

Об'єкт `TenantDailyUsage` **не** є в режимі реального часу і може відставати від фактичного використання на декілька хвилин.

Структура об'єкта `TenantDailyUsage` має такий вигляд:

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
    /** Ігнорується для білінгу. **/
    ignored: boolean
}
[inline-code-end]

---