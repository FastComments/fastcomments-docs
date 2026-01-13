Обектът `TenantDailyUsage` представлява използването за tenant за даден ден. Ако няма активност за даден tenant за даден
ден, този ден няма да има обект `TenantDailyUsage`.

Обектът `TenantDailyUsage` **не** е в реално време и може да е с минути назад от действителното използване.

Структурата на обекта `TenantDailyUsage` е следната:

[inline-code-attrs-start title = 'Структура на TenantDailyUsage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** Ignored for billing. **/
    ignored: boolean
}
[inline-code-end]
