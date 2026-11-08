`TenantPackage` एक `Tenant` के लिए उपलब्ध पैकेज की जानकारी परिभाषित करता है। एक टेनेंट के पास कई पैकेज उपलब्ध हो सकते हैं, लेकिन किसी दिए गए समय पर केवल एक ही उपयोग में होता है।

जब तक एक `Tenant` का `packageId` किसी मान्य `TenantPackage` की ओर इंगित नहीं करता, तब तक उसे किसी भी प्रोडक्ट के लिए उपयोग नहीं किया जा सकता।

`TenantPackage` ऑब्जेक्ट दो प्रकार के होते हैं:

1. निश्चित मूल्य वाले पैकेज - जहाँ `hasFlexPricing` false होता है।
2. लचीला मूल्य निर्धारण (Flexible pricing) - जहाँ `hasFlexPricing` true होता है।

दोनों स्थितियों में, पैकेज का उपयोग करने वाले खाते पर सीमाएँ परिभाषित की जाती हैं, हालाँकि Flex के साथ टेनेंट से एक आधार मूल्य के साथ-साथ उनके उपयोग के अनुसार शुल्क लिया जाता है, जो `flex*` पैरामीटर द्वारा परिभाषित होता है।

एक टेनेंट के पास कई टेनेंट पैकेज हो सकते हैं और वह [Billing Info Page.](https://fastcomments.com/auth/my-account/billing-info) से स्वयं पैकेज बदलने में सक्षम हो सकता है।

यदि आप स्वयं टेनेंट्स के लिए बिलिंग संभालेंगे, तो भी आपको प्रत्येक टेनेंट के लिए उनकी सीमाएँ परिभाषित करने हेतु एक पैकेज परिभाषित करना होगा। बस `Tenant` पर `billingHandledExternally` को `true` सेट करें और वे स्वयं अपनी बिलिंग जानकारी, या सक्रिय पैकेज, बदलने में सक्षम नहीं होंगे।

आप पैरेंट टेनेंट से अधिक सीमाओं वाले पैकेज नहीं बना सकते।

`TenantPackage` ऑब्जेक्ट की संरचना इस प्रकार है:

[inline-code-attrs-start title = 'TenantPackage संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    flexSSOUserCostCents?: null // प्रति सामान्य SSO उपयोगकर्ता लागत (एडमिन/मॉडरेटर अनुमतियों के बिना)
    flexSSOUserUnit?: null
    flexAPICreditCostCents?: null
    flexAPICreditUnit?: null
    flexModeratorCostCents?: null
    flexModeratorUnit?: null
    flexAdminCostCents?: null
    flexAdminUnit?: null
    flexDomainCostCents?: null
    flexDomainUnit?: null
    flexSSOAdminCostCents?: null // एडमिन अनुमतियों वाले प्रति SSO उपयोगकर्ता लागत (isAccountOwner या isAdminAdmin फ्लैग)
    flexSSOAdminUnit?: null
    flexSSOModeratorCostCents?: null // मॉडरेटर अनुमतियों वाले प्रति SSO उपयोगकर्ता लागत (isCommentModeratorAdmin फ्लैग)
    flexSSOModeratorUnit?: null
    flexMinimumCostCents?: null
}
[inline-code-end]