Un objeto `TenantDailyUsage` representa el uso para un inquilino en un día dado. Si no hubo actividad para un inquilino dado en un día
dado, ese día no tendrá un objeto `TenantDailyUsage`.

El objeto `TenantDailyUsage` **no** es en tiempo real y puede estar minutos detrás del uso real.

La estructura del objeto `TenantDailyUsage` es la siguiente:

[inline-code-attrs-start title = 'Estructura de TenantDailyUsage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
