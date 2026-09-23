A `Poll` je priložen komentaru, umjesto da bude samostalni objekt. Stvara se zajedno s komentarom  
(pogledajte `POST /api/v1/comments`), ili se kasnije može dodati postojećem komentaru pomoću `PUT /api/v1/polls/:commentId`.

Broj glasova se čuva na samoj anketi, pa čitanje ankete daje rezultate bez potrebe za dodatnim zbrajanjem. Pojedinačni glasovi koji stoje iza tih brojeva su objekti `PollVote`.

Svaka opcija ima `id` koji se generira prilikom stvaranja ankete. Taj id koristite za glasanje, za preimenovanje opcije i za zadržavanje opcije (i njenih glasova) kada `PUT`-ate anketu s dodanim ili uklonjenim opcijama. To je jedini siguran način referenciranja opcije – nikada ne koristite njen položaj u popisu.

[inline-code-attrs-start title = 'Struktura ankete'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

- Pitanje je obavezno i smije imati najviše 200 znakova.
- Ankete moraju imati između 2 i 10 opcija.
- Oznaka opcije je obavezna, smije imati najviše 100 znakova i mora biti jedinstvena unutar ankete (ignorirajući veličinu slova).
- `closesAt` mora biti u budućnosti kada se anketa kreira. Za trenutno zatvaranje ankete, `PATCH`‑ajte je s datumom u prošlosti.

### Site Settings

Ankete poštuju konfiguraciju vaše stranice, koju možete promijeniti pod **Customize Widget**:

- Ankete moraju biti omogućene prije nego što se anketa može stvoriti, inače API odgovara s `polls-disabled`.
- Glasanje se može ograničiti na prijavljene korisnike; u tom slučaju glas poslan samo s `anonUserId` odbija se s `poll-login-required`.