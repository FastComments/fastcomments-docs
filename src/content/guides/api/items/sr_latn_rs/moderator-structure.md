Objekat `Moderator` predstavlja konfiguraciju za moderatora.

Postoje tri vrste moderatora:

1. Administrator korisnici koji imaju oznaku `isCommentModeratorAdmin`.
2. SSO korisnici sa oznakom `isCommentModeratorAdmin`.
3. Obični komentatori, odnosno FastComments.com korisnici, koji su pozvani kao Moderatori.

Struktura `Moderator` se koristi za predstavljanje stanja moderacije za slučaj upotrebe `3`.

Ako želite da pozovete korisnika da postane moderator, putem API-ja koristite `Moderator` API kreiranjem `Moderator` i `inviting` njih.

Ako korisnik nema FastComments.com nalog, mejl sa pozivnicom će im pomoći da se podese. Ako već imaju nalog, biće im dodeljen pristup za moderaciju vašeg tenanta i `Moderator` object's `userId` biće ažuriran da pokazuje na njihovog korisnika. Nećete imati API pristup njihovom korisniku, jer u tom slučaju nalog pripada njima i njime upravlja FastComments.com.

Ako vam je potrebno potpuno upravljanje nalogom korisnika, preporučujemo ili korišćenje SSO, ili dodavanje njih kao a [Korisnik tenanta](https://fastcomments.com/auth/my-account/users) i potom dodavanje `Moderator` objekta da biste pratili njihove statistike.

Struktura `Moderator` može se koristiti kao mehanizam za praćenje statistike za slučajeve upotrebe `1` i `2`. Nakon kreiranja korisnika, dodajte `Moderator` objekat sa definisanim `userId` i njihove statistike će biti praćene na [Stranici moderatora komentara](https://fastcomments.com/auth/my-account/moderate-comments/moderators).

Struktura za `Moderator` objekat je sledeća:

[inline-code-attrs-start title = 'Struktura Moderatora'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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