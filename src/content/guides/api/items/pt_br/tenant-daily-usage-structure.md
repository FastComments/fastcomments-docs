Um objeto `TenantDailyUsage` representa o uso para um tenant em um determinado dia. Se não houve atividade para um tenant em um determinado
dia, esse dia não terá um objeto `TenantDailyUsage`.

O objeto `TenantDailyUsage` é **NÃO** em tempo real e pode estar com atraso de alguns minutos em relação ao uso real.

A estrutura do objeto `TenantDailyUsage` é a seguinte:

[inline-code-attrs-start title = 'Estrutura do TenantDailyUsage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** Ignorado para faturamento. **/
    ignored: boolean
}
[inline-code-end]

---