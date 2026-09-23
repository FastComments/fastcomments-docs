A `Poll` je prikačen uz komentar, a ne predstavlja zaseban objekat. Kreira se zajedno sa komentarom (pogledajte `POST /api/v1/comments`), ili se naknadno dodaje postojećem komentaru pomoću `PUT /api/v1/polls/:commentId`.

Broj glasova se čuva na samoj anketi, tako da čitanjem ankete dobijate rezultate bez potrebe za sabiranjem. Pojedinačni glasovi koji stoje iza tih brojeva su objekti `PollVote`.

Svaka opcija ima `id` koji se generiše prilikom kreiranja ankete. Taj id se koristi za glasanje, za promenu naziva opcije i za zadržavanje opcije (i njenih glasova) kada `PUT`‑ujete anketu sa dodatim ili uklonjenim opcijama. To je jedini siguran način da se referišete na opciju – nikada ne na njen položaj u listi.

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

- Pitanje je obavezno i može imati najviše 200 znakova.
- Ankete moraju imati između 2 i 10 opcija.
- Naziv opcije je obavezan, može imati najviše 100 znakova i mora biti jedinstven unutar ankete (bez obzira na veličinu slova).
- `closesAt` mora biti u budućnosti prilikom kreiranja ankete. Da biste odmah zatvorili anketu, `PATCH`‑ujte je sa datumom u prošlosti.

### Site Settings

Ankete poštuju konfiguraciju vašeg sajta, koju možete promeniti u sekciji **Customize Widget**:

- Ankete moraju biti omogućene pre nego što se anketa može kreirati, inače API vraća `polls-disabled`.
- Glasanje može biti ograničeno na prijavljene korisnike, u kom slučaju glas poslat samo sa `anonUserId` se odbija sa `poll-login-required`.