Objekt `Subscription` predstavlja naročnino za uporabnika.

`Subscription` objekti se ustvarijo, ko uporabnik klikne zvonček za obvestila v pripomočku za komentarje in izbere "Naroči se na to stran".

Naročnine je mogoče ustvariti tudi prek API-ja.

Prisotnost objekta `Subscription` povzroči ustvarjanje objektov `Notification` in pošiljanje e-poštnih sporočil, ko so na korenu povezane strani
za katero je `Subscription`, objavljeni novi komentarji. Pošiljanje e-pošte je odvisno od tipa uporabnika. Pri običajnih uporabnikih to določa `optedInNotifications`. Pri SSO uporabnikih to določa `optedInSubscriptionNotifications`. Upoštevajte, da nekateri programi morda nimajo koncepta spletno dostopne strani, v tem primeru preprosto nastavite `urlId` na id elementa, na katerega se naročate (isto vrednost za `urlId`, ki bi jo posredovali v pripomoček za komentarje).

Struktura objekta `Subscription` je naslednja:

[inline-code-attrs-start title = 'Struktura objekta Subscription'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Subscription {
    id: string
    tenantId: string
    /** Pri SSO je uporabniški id v formatu `<tenant id>:<user id>`. **/
    userId: string
    anonUserId?: string
    urlId: string
    url?: string
    pageTitle?: string
    createdAt: string // niz datuma
}
[inline-code-end]

---