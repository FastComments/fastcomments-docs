`TenantDailyUsage` 객체는 특정 날짜의 테넌트 사용량을 나타냅니다. 특정 날짜에 테넌트의 활동이 없었다면, 그 날짜에는 `TenantDailyUsage` 객체가 존재하지 않습니다.

`TenantDailyUsage` 객체는 실시간이 **아니며**, 실제 사용량보다 몇 분 늦을 수 있습니다.

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
    /** Ignored for billing. **/
    ignored: boolean
}
[inline-code-end]