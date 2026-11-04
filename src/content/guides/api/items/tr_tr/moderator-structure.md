Bir `Moderator` nesnesi, bir moderatörün yapılandırmasını temsil eder.

Üç tür moderatör vardır:

1. `isCommentModeratorAdmin` bayrağına sahip Yönetici kullanıcılar.
2. `isCommentModeratorAdmin` bayrağına sahip SSO Kullanıcıları.
3. Moderatör olarak davet edilen normal yorumcular veya FastComments.com kullanıcıları.

`Moderator` yapısı, `3` numaralı kullanım senaryosunun Moderasyon Durumunu temsil etmek için kullanılır.

Bir kullanıcıyı API üzerinden moderatör olarak davet etmek isterseniz, bir `Moderator` oluşturarak ve onları `inviting` (davet ederek) `Moderator` API'sini kullanın.

Kullanıcının FastComments.com hesabı yoksa, davet e-postası kurulum yapmasına yardımcı olacaktır. Zaten bir hesabı varsa, kiracınıza (tenant) moderasyon erişimi verilecek ve `Moderator` nesnesinin `userId` alanı, kullanıcısını işaret edecek şekilde güncellenecektir. Bu durumda kullanıcı onlara ait olduğu ve FastComments.com tarafından yönetildiği için, kullanıcısına API erişiminiz olmayacaktır.

Kullanıcının hesabının tam yönetimine ihtiyacınız varsa, ya SSO kullanmanızı ya da onları bir [Tenant User](https://fastcomments.com/auth/my-account/users) olarak eklemenizi ve ardından istatistiklerini takip etmek için bir `Moderator` nesnesi eklemenizi öneririz.

`Moderator` yapısı, `1` ve `2` numaralı kullanım senaryoları için bir istatistik takip mekanizması olarak kullanılabilir. Kullanıcıyı oluşturduktan sonra, `userId` alanı tanımlanmış bir `Moderator` nesnesi ekleyin; böylece istatistikleri [Comment Moderators Page](https://fastcomments.com/auth/my-account/moderate-comments/moderators) sayfasında takip edilecektir.

`Moderator` nesnesinin yapısı aşağıdaki gibidir:

[inline-code-attrs-start title = 'Moderator Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Moderator {
    name: string
    email: string
    tenantId: string
    userId?: string|null
    acceptedInvite?: boolean
    markReviewedCount?: number
    deletedCount?: number
    markedSpamCount?: number
    approvedCount?: number
    editedCount?: number
    bannedCount?: number
    createdAt: string
    moderationGroupIds?: string[]|null
}
[inline-code-end]