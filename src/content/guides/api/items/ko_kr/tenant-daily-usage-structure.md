A `TenantDailyUsage` 객체는 특정 날짜에 대한 테넌트의 사용량을 나타냅니다. 특정 테넌트가 해당 날짜에 활동이 없었다면 그 날짜에는 `TenantDailyUsage` 객체가 생성되지 않습니다.

The `TenantDailyUsage` object is **not** real time and may be minutes behind actual usage.

`TenantDailyUsage` 객체의 구조는 다음과 같습니다:

[inline-code-attrs-start title = 'TenantDailyUsage 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** 청구에서 무시됩니다. **/
    ignored: boolean
}
[inline-code-end]

---