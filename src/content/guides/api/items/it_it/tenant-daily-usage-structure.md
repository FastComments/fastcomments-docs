Un oggetto `TenantDailyUsage` rappresenta l'utilizzo per un tenant in un dato giorno. Se non c'è stata attività per un tenant in un determinato giorno, quel giorno non avrà un oggetto `TenantDailyUsage`.

L'oggetto `TenantDailyUsage` **non** è in tempo reale e può essere indietro di alcuni minuti rispetto all'utilizzo effettivo.

La struttura dell'oggetto `TenantDailyUsage` è la seguente:

[inline-code-attrs-start title = 'Struttura di TenantDailyUsage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** Ignorato per la fatturazione. **/
    ignored: boolean
}
[inline-code-end]

---