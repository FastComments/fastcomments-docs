`Poll` je vezan uz komentar, a ne da je samostalan objekt. Stvara se zajedno s komentarom
(vidi `POST /api/v1/comments`), ili se kasnije dodaje postojećem komentaru putem `PUT /api/v1/polls/:commentId`.

Brojevi glasova čuvaju se na samom anketu, pa čitanje ankete daje rezultate bez potrebe za zbrajanjem
bilo čega. Pojedinačni glasovi iza tih brojeva su `PollVote` objekti.

Svaka opcija ima `id` koji se generira prilikom stvaranja ankete. Taj id koristite za davanje glasa,
za preimenovanje opcije i za zadržavanje opcije (i njenih glasova) kada `PUT`-ate anketu s dodanim ili
uklonjenim opcijama. To je jedini siguran način referenciranja opcije - nikada njen položaj u listi.

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

### Ograničenja

- Pitanje je obavezno i ima najviše 200 znakova.
- Anketa ima između 2 i 10 opcija.
- Oznaka opcije je obavezna, ima najviše 100 znakova i mora biti jedinstvena unutar ankete (zanemarujući velika/mala slova).
- `closesAt` mora biti u budućnosti prilikom stvaranja ankete. Za trenutno zatvaranje ankete, `PATCH`-ajte je s
  datumom u prošlosti.

### Postavke stranice

Ankete poštuju konfiguraciju vaše stranice, koju možete promijeniti pod Customize Widget:

- Ankete moraju biti omogućene prije stvaranja ankete, inače API odgovara s `polls-disabled`.
- Glasovanje može biti ograničeno na prijavljene korisnike, u kojem slučaju glas poslan samo s `anonUserId` se odbija
  s `poll-login-required`.