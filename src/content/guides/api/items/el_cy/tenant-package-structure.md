Το `TenantPackage` ορίζει τις πληροφορίες πακέτου που είναι διαθέσιμες σε έναν `Tenant`. Ένας ενοικιαστής μπορεί να έχει πολλά διαθέσιμα πακέτα, αλλά μόνο
ένα σε χρήση σε δεδομένη στιγμή.

Ένας `Tenant` δεν μπορεί να χρησιμοποιηθεί για κανένα προϊόν μέχρι το `packageId` του να δείχνει σε ένα έγκυρο `TenantPackage`.

Υπάρχουν δύο τύποι αντικειμένων `TenantPackage`:

1. Πακέτα σταθερής τιμολόγησης - όπου το `hasFlexPricing` είναι false.
2. Ευέλικτη τιμολόγηση - όπου το `hasFlexPricing` είναι true.

Και στις δύο περιπτώσεις τα όρια ορίζονται στον λογαριασμό που χρησιμοποιεί το πακέτο, ωστόσο με το Flex ο ενοικιαστής χρεώνεται μια βασική τιμή συν
αυτό που χρησιμοποίησε, που ορίζεται από τις παραμέτρους `flex*`.

Ένας ενοικιαστής μπορεί να έχει πολλαπλά πακέτα ενοικιαστή και να έχει τη δυνατότητα να αλλάξει το πακέτο ο ίδιος από τη [Σελίδα Πληροφοριών Χρέωσης.](https://fastcomments.com/auth/my-account/billing-info)

Αν θα χειρίζεστε τη χρέωση για τους ενοικιαστές εσείς, θα χρειαστεί ακόμα να ορίσετε ένα πακέτο για κάθε ενοικιαστή για να ορίσετε τα όριά τους. Απλά ορίστε το `billingHandledExternally` σε `true` στον `Tenant` και δεν θα
μπορούν να αλλάξουν τις πληροφορίες χρέωσής τους, ή το ενεργό πακέτο, οι ίδιοι.

Δεν μπορείτε να δημιουργήσετε πακέτα με υψηλότερα όρια από τον γονικό ενοικιαστή.

Η δομή για το αντικείμενο `TenantPackage` είναι η ακόλουθη:

[inline-code-attrs-start title = 'Δομή TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface TenantPackage {
    id: string
    name: string
    tenantId: string
    createdAt: string
    monthlyCostUSD: number
    yearlyCostUSD: number
    monthlyStripePlanId?: string
    yearlyStripePlanId?: string
    maxMonthlyPageLoads: number
    maxMonthlyAPICredits: number
    maxMonthlyComments: number
    maxConcurrentUsers: number
    maxTenantUsers: number
    maxSSOUsers: number
    maxModerators: number
    maxDomains: number
    maxWhiteLabeledTenants: number
    hasWhiteLabeling: boolean
    hasDebranding: boolean
    forWhoText: string
    featureTaglines: string[]
    hasAuditing: boolean
    hasFlexPricing: boolean
    flexPageLoadCostCents?: null
    flexPageLoadUnit?: null
    flexCommentCostCents?: null
    flexCommentUnit?: null
    flexSSOUserCostCents?: null // Cost per regular SSO user (without admin/moderator permissions)
    flexSSOUserUnit?: null
    flexAPICreditCostCents?: null
    flexAPICreditUnit?: null
    flexModeratorCostCents?: null
    flexModeratorUnit?: null
    flexAdminCostCents?: null
    flexAdminUnit?: null
    flexDomainCostCents?: null
    flexDomainUnit?: null
    flexSSOAdminCostCents?: null // Cost per SSO user with admin permissions (isAccountOwner or isAdminAdmin flags)
    flexSSOAdminUnit?: null
    flexSSOModeratorCostCents?: null // Cost per SSO user with moderator permissions (isCommentModeratorAdmin flag)
    flexSSOModeratorUnit?: null
    flexMinimumCostCents?: null
}
[inline-code-end]
