`Subscription` objekat predstavlja pretplatu za korisnika.

`Subscription` objekti se kreiraju kada korisnik klikne na ikonu zvona za obaveštenja u komentarskom widgetu i klikne na "Pretplati se na ovu stranicu".

Pretplate se takođe mogu kreirati putem API-ja.

Pojava `Subscription` objekta izaziva generisanje `Notification` objekata i slanje mejlova kada se ostave novi komentari na root-u povezane stranice
za koju je `Subscription`. Slanje mejlova zavisi od tipa korisnika. Za obične korisnike ovo zavisi od `optedInNotifications`. Za SSO korisnike ovo zavisi od `optedInSubscriptionNotifications`. Imajte na umu da neke aplikacije možda nemaju koncept veb-pristupačne stranice, u kom slučaju jednostavno postavite `urlId` na
id stavke na koju se pretplaćujete (ista vrednost za `urlId` koju biste prosledili komentarskom widgetu).

Struktura za `Subscription` objekat je sledeća:

[inline-code-attrs-start title = 'Struktura Subscription objekta'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Subscription {
    id: string
    tenantId: string
    /** Kod SSO, user id je u formatu `<tenant id>:<user id>`. **/
    userId: string
    anonUserId?: string
    urlId: string
    url?: string
    pageTitle?: string
    createdAt: string // string datuma
}
[inline-code-end]

---