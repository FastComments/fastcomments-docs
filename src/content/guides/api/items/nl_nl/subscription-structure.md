Een `Subscription`-object vertegenwoordigt een abonnement voor een gebruiker.

`Subscription`-objecten worden aangemaakt wanneer een gebruiker op de meldingsbel in de reactie-widget klikt en op "Abonneer op deze pagina" klikt.

Abonnementen kunnen ook via de API worden aangemaakt.

Het hebben van een `Subscription`-object zorgt ervoor dat er `Notification`-objecten worden gegenereerd en e-mails worden verzonden wanneer er nieuwe reacties worden geplaatst op de root van de bijbehorende pagina waarvoor de `Subscription` geldt. Het verzenden van e-mails hangt af van het type gebruiker. Voor reguliere gebruikers is dit afhankelijk van `optedInNotifications`. Voor SSO-gebruikers is dit afhankelijk van `optedInSubscriptionNotifications`. Let op dat sommige applicaties mogelijk niet het concept van een via het web toegankelijke pagina hebben; in dat geval stelt u eenvoudig `urlId` in op de id van het item waarop u zich abonneert (dezelfde waarde voor `urlId` die u aan de reactie-widget zou doorgeven).

De structuur van het `Subscription`-object is als volgt:

[inline-code-attrs-start title = 'Structuur van Subscription'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Subscription {
    id: string
    tenantId: string
    /** Bij SSO is de gebruikers-id in het formaat `<tenant id>:<user id>`. **/
    userId: string
    anonUserId?: string
    urlId: string
    url?: string
    pageTitle?: string
    createdAt: string // datumstring
}
[inline-code-end]