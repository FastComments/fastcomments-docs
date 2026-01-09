Objekt `Subscription` predstavlja pretplatu za korisnika.

`Subscription` objekti se stvaraju kada korisnik klikne ikonu za obavijest u widgetu komentara i klikne "Pretplati se na ovu stranicu".

Pretplate se također mogu stvoriti putem API-ja.

Posjedovanje objekta `Subscription` uzrokuje generiranje objekata `Notification` i slanje e-poruka kada se ostave novi komentari na korijenu pridružene stranice na koju se odnosi `Subscription`.
Slanje e-poruka ovisi o tipu korisnika. Za obične korisnike to ovisi o `optedInNotifications`. Za SSO korisnike to ovisi o `optedInSubscriptionNotifications`. Imajte na umu da neke aplikacije možda nemaju koncept web-pristupačne stranice, u kojem slučaju jednostavno postavite `urlId` na id stavke na koju se pretplaćujete (ista vrijednost za `urlId` koju biste proslijedili widgetu komentara).

The structure for the `Subscription` object is as follows:

[inline-code-attrs-start title = 'Struktura objekta Subscription'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Subscription {
    id: string
    tenantId: string
    /** SSO: korisnički id je u formatu `<tenant id>:<user id>`. **/
    userId: string
    anonUserId?: string
    urlId: string
    url?: string
    pageTitle?: string
    createdAt: string // string datuma
}
[inline-code-end]

---