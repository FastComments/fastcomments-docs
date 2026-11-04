Objekt `Subscription` predstavlja pretplatu za korisnika.

Objekti `Subscription` stvaraju se kada korisnik klikne na zvonce za obavijesti u widgetu za komentare i klikne "Subscribe to this page".

Pretplate se mogu stvoriti i putem API-ja.

Postojanje objekta `Subscription` uzrokuje generiranje objekata `Notification` i slanje e-pošte kada se novi komentari ostave na korijenu povezane stranice
za koju je `Subscription` vezan. Slanje e-pošte ovisi o vrsti korisnika. Za obične korisnike to ovisi o `optedInNotifications`. Za SSO korisnike to ovisi o `optedInSubscriptionNotifications`. Imajte na umu da neke aplikacije možda nemaju koncept web-dostupne stranice, u kojem slučaju jednostavno postavite `urlId` na
id stavke na koju se pretplaćujete (ista vrijednost za `urlId` koju biste proslijedili widgetu za komentare).

Struktura objekta `Subscription` je sljedeća:

[inline-code-attrs-start title = 'Struktura pretplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Subscription {
    id: string
    tenantId: string
    /** With SSO, the user id is in the format `<tenant id>:<user id>`. **/
    userId: string
    anonUserId?: string
    urlId: string
    url?: string
    pageTitle?: string
    createdAt: string // date string
}
[inline-code-end]