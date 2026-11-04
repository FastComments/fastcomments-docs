Een `PollVote` is het antwoord van één persoon op een poll. De aantallen die op de poll zelf worden getoond worden synchroon gehouden met deze,
dus je hebt deze alleen nodig als je wilt weten *wie* op wat heeft gestemd, in plaats van de totalen.

Een stemmer heeft maximaal één stem per poll. Als er opnieuw gestemd wordt, wordt de bestaande stem verplaatst naar de nieuwe optie in plaats van
dat er een tweede wordt toegevoegd, en `updatedAt` registreert wanneer dat gebeurde.

`voterId` is de `userId` wanneer de stemmer was ingelogd, en anders de `anonUserId`.

[inline-code-attrs-start title = 'PollVote Structuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVote {
    id: string
    tenantId: string
    commentId: string
    urlId: string
    /** De userId wanneer de stemmer was ingelogd, anders de anonUserId. **/
    voterId: string
    optionId: string
    createdAt: string
    /** Wanneer de stemmer zijn of haar stem voor het laatst naar een andere optie heeft verplaatst. **/
    updatedAt?: string
}
[inline-code-end]

### Privacy

De `privacy`-instelling van de poll geldt voor deze API op dezelfde manier als in de comment-widget:

- **Anoniem** (de standaardinstelling): niemand kan zien hoe iemand heeft gestemd, dus de stemmen kunnen niet worden uitgelezen.
  `GET /api/v1/poll-votes` en `GET /api/v1/poll-votes/:id` reageren met `poll-anonymous`. De aantallen van de poll zijn
  nog steeds beschikbaar via `GET /api/v1/polls/:commentId`.
- **Admins en moderatoren**: je API-sleutel behoort toe aan de admin van je site, dus deze kan de stemmen uitlezen.
- **Iedereen**: de stemmen kunnen worden uitgelezen.

De privacy van een poll kan worden versmald, maar niet worden verruimd zodra er stemmen zijn.