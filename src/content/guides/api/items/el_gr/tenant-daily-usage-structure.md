Ένα αντικείμενο `TenantDailyUsage` αντιπροσωπεύει τη χρήση για έναν ενοικιαστή σε μια δεδομένη ημέρα. Αν δεν υπήρξε δραστηριότητα για έναν δεδομένο ενοικιαστή σε μια δεδομένη
ημέρα, αυτή η ημέρα δεν θα έχει αντικείμενο `TenantDailyUsage`.

Το αντικείμενο `TenantDailyUsage` **δεν** είναι σε πραγματικό χρόνο και μπορεί να υστερεί κατά λεπτά από την πραγματική χρήση.

Η δομή για το αντικείμενο `TenantDailyUsage` είναι η ακόλουθη:

[inline-code-attrs-start title = 'Δομή TenantDailyUsage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
