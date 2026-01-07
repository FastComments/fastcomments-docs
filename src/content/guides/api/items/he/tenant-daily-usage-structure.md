אובייקט `TenantDailyUsage` מייצג את השימוש של שוכר ביום נתון. אם לא הייתה פעילות לשוכר נתון ביום נתון,
אותו יום לא יהיה לו אובייקט `TenantDailyUsage`.

אובייקט `TenantDailyUsage` **אינו** בזמן אמת ועשוי להיות מספר דקות מאחורי השימוש בפועל.

המבנה עבור אובייקט `TenantDailyUsage` הוא כדלקמן:

[inline-code-attrs-start title = 'מבנה TenantDailyUsage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
