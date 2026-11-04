Objekt `Moderator` predstavlja konfiguracijo moderatorja.

Obstajajo trije tipi moderatorjev:

1. Skrbniški uporabniki, ki imajo zastavico `isCommentModeratorAdmin`.
2. SSO uporabniki z zastavico `isCommentModeratorAdmin`.
3. Redni komentatorji ali uporabniki FastComments.com, ki so povabljeni kot moderatorji.

Struktura `Moderator` se uporablja za predstavitev stanja moderacije primera uporabe `3`.

Če želite povabiti uporabnika, da postane moderator, prek API-ja, uporabite API `Moderator` tako, da ustvarite `Moderator` in jih `inviting`.

Če uporabnik nima računa na FastComments.com, mu povabilo po e-pošti pomaga pri nastavitvi. Če račun že ima, bo
dobil moderatorski dostop do vašega najemnika (tenant) in `userId` objekta `Moderator` bo posodobljen, da kaže na njegovega uporabnika. Do njegovega uporabnika ne boste imeli API
dostopa, saj v tem primeru pripada samemu sebi in ga upravlja FastComments.com.

Če želite popolnoma upravljati uporabnikov račun, priporočamo, da uporabite SSO ali da ga dodate kot [Uporabnika najemnika](https://fastcomments.com/auth/my-account/users) in
nato dodate objekt `Moderator` za sledenje njegovim statistikam.

Struktura `Moderator` se lahko uporablja kot mehanizem za sledenje statistikam za primera uporabe `1` in `2`. Po ustvarjanju uporabnika dodajte objekt `Moderator`
z definiranim `userId` in njegove statistike bodo sledene na [Strani moderatorjev komentarjev](https://fastcomments.com/auth/my-account/moderate-comments/moderators).

Struktura objekta `Moderator` je naslednja:

[inline-code-attrs-start title = 'Struktura Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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