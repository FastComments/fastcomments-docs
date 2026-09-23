Een `Poll` is gekoppeld aan een reactie, in plaats van een eigen object te zijn. Deze wordt aangemaakt samen met de reactie (zie `POST /api/v1/comments`), of later toegevoegd aan een bestaande reactie met `PUT /api/v1/polls/:commentId`.

De stemtellingen worden bewaard op de poll zelf, zodat het lezen van een poll je de resultaten geeft zonder dat je iets hoeft op te tellen. De individuele stemmen achter die tellingen zijn `PollVote` objecten.

Elke optie heeft een `id` die wordt gegenereerd wanneer de poll wordt aangemaakt. Die id gebruik je om een stem uit te brengen, om een optie een nieuw label te geven, en om een optie (en de stemmen ervan) te behouden wanneer je de poll `PUT` met toegevoegde of verwijderde opties. Het is de enige veilige manier om naar een optie te verwijzen – nooit naar de positie in de lijst.

[inline-code-attrs-start title = 'Pollstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPollOption {
    id: string
    label: string
    votes: number
}

interface CommentPoll {
    question: string
    options: CommentPollOption[]
    totalVotes: number
    /** When set and in the past, the poll is closed and no longer accepts votes. **/
    closesAt?: string | null
    /** 0 anonymous (the default), 1 admins and moderators, 2 everyone. Absent means anonymous. **/
    privacy?: 0 | 1 | 2 | null
    /** When true, the counts are hidden from anyone who has not voted yet. Absent means false. **/
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

### Limits

- Een vraag is verplicht en mag maximaal 200 tekens bevatten.
- Een poll heeft tussen de 2 en 10 opties.
- Een optielabel is verplicht, mag maximaal 100 tekens bevatten en moet uniek zijn binnen de poll (hoofdletterongevoelig).
- `closesAt` moet in de toekomst liggen wanneer de poll wordt aangemaakt. Om een poll onmiddellijk te sluiten, `PATCH` je deze met een datum in het verleden.

### Site Settings

Polls volgen de configuratie van je site, die je kunt aanpassen onder Customize Widget:

- Polls moeten ingeschakeld zijn voordat een poll kan worden aangemaakt, anders reageert de API met `polls-disabled`.
- Stemmen kan worden beperkt tot ingelogde gebruikers; in dat geval wordt een stem die alleen een `anonUserId` bevat afgewezen met `poll-login-required`.