`TenantDailyUsage` オブジェクトは、特定の日におけるテナントの使用量を表します。特定のテナントにその日にアクティビティがなかった場合、その日には `TenantDailyUsage` オブジェクトは存在しません。

`TenantDailyUsage` オブジェクトは **リアルタイムではありません**。実際の使用状況より数分遅れることがあります。

`TenantDailyUsage` オブジェクトの構造は次のとおりです:

[inline-code-attrs-start title = 'TenantDailyUsage の構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** 課金対象外。 **/
    ignored: boolean
}
[inline-code-end]

---