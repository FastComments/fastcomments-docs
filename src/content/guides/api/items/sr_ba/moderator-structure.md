Objekat `Moderator` predstavlja konfiguraciju za moderatora.

Postoje tri tipa moderatora:

1. Administrator korisnici koji imaju zastavicu `isCommentModeratorAdmin`.
2. SSO korisnici sa zastavicom `isCommentModeratorAdmin`.
3. Redovni komentatori, ili FastComments.com korisnici, koji su pozvani kao moderatori.

Struktura `Moderator` se koristi za predstavljanje stanja moderacije u slučaju upotrebe `3`.

Ako želite pozvati korisnika da bude moderator putem API-ja, koristite `Moderator` API kreiranjem `Moderator` i `inviting` njima.

Ako korisnik nema FastComments.com nalog, pozivni email će im pomoći da se podese. Ako već imaju nalog, biće im dodan moderacijski pristup vašem tenant-u i `Moderator` objektu će se ažurirati `userId` da pokazuje na njihovog korisnika. Nećete imati API pristup njihovom korisniku, jer u ovom slučaju nalog pripada njima i upravlja ga FastComments.com.

Ako zahtijevate potpunu kontrolu nad korisničkim nalogom, preporučujemo ili korištenje SSO-a, ili dodavanje korisnika kao [Korisnik tenanta](https://fastcomments.com/auth/my-account/users) i zatim dodavanje `Moderator` objekta za praćenje njihove statistike.

Struktura `Moderator` može se koristiti kao mehanizam za praćenje statistike za slučajeve upotrebe `1` i `2`. Nakon kreiranja korisnika, dodajte `Moderator` objekat sa definisanim `userId` i njihove statistike će biti praćene na [Stranici za moderatore komentara](https://fastcomments.com/auth/my-account/moderate-comments/moderators).

Struktura za `Moderator` objekat je sljedeća:

[inline-code-attrs-start title = 'Struktura moderatora'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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