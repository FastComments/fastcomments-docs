Et `Subscription`-objekt repræsenterer et abonnement for en bruger.

`Subscription`-objekter oprettes, når en bruger klikker på notifikationsklokken i kommentar-widget'en og klikker "Abonner på denne side".

Abonnementer kan også oprettes via API'et.

At have et `Subscription`-objekt medfører, at `Notification`-objekter genereres, og e-mails sendes, når nye kommentarer efterlades på roden af den tilknyttede side,
som `Subscription` er for. Afsendelse af e-mails afhænger af brugertypen. For almindelige brugere afhænger dette af `optedInNotifications`. For SSO-brugere afhænger dette af `optedInSubscriptionNotifications`. Bemærk at nogle applikationer måske ikke har konceptet om en web-tilgængelig side, i hvilket tilfælde du blot skal sætte `urlId` til
id'et for det element, du abonnerer på (samme værdi for `urlId` som du ville sende til kommentar-widget'en).

Strukturen for `Subscription`-objektet er som følger:

[inline-code-attrs-start title = 'Subscription Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
