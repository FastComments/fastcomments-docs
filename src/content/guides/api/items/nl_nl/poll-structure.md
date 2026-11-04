Een `Poll` is gekoppeld aan een comment, in plaats van dat het een zelfstandig object is. Het wordt aangemaakt samen met de comment
(zie `POST /api/v1/comments`), of later toegevoegd aan een bestaande comment via `PUT /api/v1/polls/:commentId`.

De stemaantallen worden bijgehouden op de poll zelf, dus het lezen van een poll geeft je de resultaten zonder dat je
iets hoeft op te tellen. De individuele stemmen achter die aantallen zijn `PollVote`-objecten.

Elke optie heeft een `id` die wordt gegenereerd wanneer de poll wordt aangemaakt. Dat id gebruik je om een stem uit te brengen,
om een optie te hernoemen, en om een optie (en de stemmen erop) te behouden wanneer je de poll `PUT` met toegevoegde of
verwijderde opties. Het is de enige veilige manier om naar een optie te verwijzen - nooit naar de positie in de lijst.

[inline-code-attrs-start title = 'Poll Structuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** Wanneer ingesteld en in het verleden, is de poll gesloten en worden er geen stemmen meer geaccepteerd. **/
    closesAt?: string | null
    /** 0 anoniem (de standaard), 1 admins en moderators, 2 iedereen. Afwezig betekent anoniem. **/
    privacy?: 0 | 1 | 2 | null
    /** Wanneer true, zijn de aantallen verborgen voor iedereen die nog niet heeft gestemd. Afwezig betekent false. **/
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

### Limieten

- Een vraag is verplicht en heeft maximaal 200 tekens.
- Een poll heeft tussen de 2 en 10 opties.
- Een optielabel is verplicht, heeft maximaal 100 tekens en moet uniek zijn binnen de poll (hoofdlettergevoeligheid genegeerd).
- `closesAt` moet in de toekomst liggen wanneer de poll wordt aangemaakt. Om een poll direct te sluiten, doe je een
  `PATCH` met een datum in het verleden.

### Site-instellingen

Polls volgen je siteconfiguratie, die je kunt aanpassen onder Customize Widget:

- Polls moeten ingeschakeld zijn voordat een poll kan worden aangemaakt, anders reageert de API met `polls-disabled`.
- Stemmen kan worden beperkt tot ingelogde gebruikers, in welk geval een stem die alleen met een `anonUserId` wordt
  verstuurd wordt afgewezen met `poll-login-required`.