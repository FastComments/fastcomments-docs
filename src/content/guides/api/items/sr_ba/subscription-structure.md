---
A `Subscription` object predstavlja pretplatu za korisnika.

`Subscription` objekti se kreiraju kada korisnik klikne na zvonce za obavještenja u widgetu za komentare i izabere "Pretplati se na ovu stranicu".

Pretplate se također mogu kreirati putem API-ja.

Posjedovanje `Subscription` objekta uzrokuje generisanje `Notification` objekata i slanje emailova kada se ostave novi komentari na korijenu pridružene stranice za koju je `Subscription`. Slanje emailova zavisi od tipa korisnika. Za obične korisnike to zavisi od `optedInNotifications`. Za SSO korisnike to zavisi od `optedInSubscriptionNotifications`. Imajte na umu da neke aplikacije možda nemaju pojam web-pristupačne stranice, u tom slučaju jednostavno postavite `urlId` na id stavke na koju se pretplaćujete (istu vrijednost za `urlId` koju biste proslijedili widgetu za komentare).

Struktura `Subscription` objekta je sljedeća:

[inline-code-attrs-start title = 'Struktura Subscription objekta'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Subscription {
    id: string
    tenantId: string
    /** Kod SSO, user id ima format `<tenant id>:<user id>`. **/
    userId: string
    anonUserId?: string
    urlId: string
    url?: string
    pageTitle?: string
    createdAt: string // datum u obliku stringa
}
[inline-code-end]

---