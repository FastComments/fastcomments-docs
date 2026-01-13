Un objet `TenantDailyUsage` représente l'utilisation pour un locataire sur un jour donné. S'il n'y a eu aucune activité pour un locataire donné sur un jour
donné, ce jour n'aura pas d'objet `TenantDailyUsage`.

L'objet `TenantDailyUsage` n'est **pas** en temps réel et peut être en retard de quelques minutes par rapport à l'utilisation réelle.

La structure pour l'objet `TenantDailyUsage` est la suivante:

[inline-code-attrs-start title = 'Structure de TenantDailyUsage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
