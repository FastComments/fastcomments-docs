Objekt `Moderator` predstavlja konfiguraciju za moderatora.

Postoje tri vrste moderatora:

1. Korisnici administratorske uloge koji imaju oznaku `isCommentModeratorAdmin`.
2. SSO korisnici s oznakom `isCommentModeratorAdmin`.
3. Obični komentatori, odnosno korisnici FastComments.com-a, koji su pozvani kao Moderatori.

Struktura `Moderator` koristi se za prikaz stanja moderiranja za slučaj upotrebe `3`.

Ako želite pozvati korisnika da postane moderator putem API-ja, upotrijebite `Moderator` API stvaranjem `Moderator` i `inviting` njih.

Ako korisnik nema račun na FastComments.com, e‑poruka s pozivnicom pomoći će im pri postavljanju računa. Ako već imaju račun, dobit će pristup moderiranju vašem tenantu i svojstvo `userId` na objektu `Moderator` bit će ažurirano tako da pokazuje na njihovog korisnika. Nećete imati API pristup njihovom korisničkom računu, jer u tom slučaju on pripada njima i upravlja ga FastComments.com.

Ako vam je potrebna potpuna kontrola nad korisničkim računom, preporučujemo ili korištenje SSO-a, ili dodavanje korisnika kao [Korisnik tenanta](https://fastcomments.com/auth/my-account/users) i potom dodavanje `Moderator` objekta za praćenje njihovih statistika.

Struktura `Moderator` može se koristiti kao mehanizam za praćenje statistike za slučajeve upotrebe `1` i `2`. Nakon stvaranja korisnika, dodajte `Moderator` objekt s definiranim `userId` i njihove će statistike biti praćene na [Stranica moderatora komentara](https://fastcomments.com/auth/my-account/moderate-comments/moderators).

Struktura objekta `Moderator` je sljedeća:

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

---