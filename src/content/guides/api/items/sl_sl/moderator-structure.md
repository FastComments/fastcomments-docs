Objekt `Moderator` predstavlja konfiguracijo za moderatorja.

Obstajajo trije tipi moderatorjev:

1. Administrator users that have the `isCommentModeratorAdmin` flag.
2. SSO Users with the `isCommentModeratorAdmin` flag.
3. Regular commenters, or FastComments.com users, that are invited as Moderators.

Struktura `Moderator` se uporablja za predstavitev stanja moderacije pri primeru uporabe `3`.

Če želite preko API-ja povabiti uporabnika, da postane moderator, uporabite `Moderator` API tako, da ustvarite `Moderator` in jih `inviting`.

Če uporabnik nima računa FastComments.com, mu bo povabilo po e-pošti pomagalo pri nastavitvi. Če že ima račun, mu bo dodeljen dostop za moderiranje do vašega tenant-a in lastnost `userId` objekta `Moderator` bo posodobljena, da kaže na njihovega uporabnika. Nimali boste API dostopa do njihovega uporabnika, saj v tem primeru pripada njim in ga upravlja FastComments.com.

Če potrebujete popolno upravljanje računa uporabnika, priporočamo uporabo SSO ali pa ga dodajte kot [Tenant User](https://fastcomments.com/auth/my-account/users) in nato dodajte objekt `Moderator` za sledenje njihovim statistikam.

Strukturo `Moderator` je mogoče uporabiti kot mehanizem za sledenje statistik v primerih uporabe `1` in `2`. Po ustvarjanju uporabnika dodajte objekt `Moderator` z določenim `userId` in njihove statistike se bodo spremljale na [Comment Moderators Page](https://fastcomments.com/auth/my-account/moderate-comments/moderators).

Struktura objekta `Moderator` je naslednja:

[inline-code-attrs-start title = 'Struktura Moderatorja'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---