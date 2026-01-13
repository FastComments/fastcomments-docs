Objekat `Subscription` predstavlja pretplatu za korisnika.

`Subscription` objekti se kreiraju kada korisnik klikne na zvonce za obaveštenja u widgetu za komentare i klikne "Subscribe to this page".

Pretplate se takođe mogu kreirati putem API-ja.

Imanje objekta `Subscription` uzrokuje kreiranje objekata `Notification` i slanje emailova kada se ostave novi komentari na korenu povezane stranice za koju je ta `Subscription`. Slanje emailova zavisi od tipa korisnika. Za obične korisnike to zavisi od `optedInNotifications`. Za SSO korisnike to zavisi od `optedInSubscriptionNotifications`. Imajte na umu da neke aplikacije možda nemaju koncept web-pristupačne stranice, u kom slučaju jednostavno postavite `urlId` na id stavke na koju se pretplaćujete (ista vrednost za `urlId` koju biste prosledili widgetu za komentare).

Struktura za objekat `Subscription` je sledeća:

[inline-code-attrs-start title = 'Struktura objekta Subscription'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Subscription {
    id: string
    tenantId: string
    /** Sa SSO-om, korisnički id je u formatu `<tenant id>:<user id>`. **/
    userId: string
    anonUserId?: string
    urlId: string
    url?: string
    pageTitle?: string
    createdAt: string // string datuma
}
[inline-code-end]

---