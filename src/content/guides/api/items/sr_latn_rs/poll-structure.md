`Poll` je vezan za komentar, a nije samostalan objekat. Kreira se zajedno sa komentarom
(vidi `POST /api/v1/comments`), ili se kasnije dodaje postojećem komentaru putem `PUT /api/v1/polls/:commentId`.

Brojevi glasova se čuvaju na samoj anketi, tako da čitanje ankete daje rezultate bez potrebe da bilo šta
sabirate. Pojedinačni glasovi iza tih brojeva su `PollVote` objekti.

Svaka opcija ima `id` koji se generiše prilikom kreiranja ankete. Taj id koristite za glasanje,
za preimenovanje opcije, i za zadržavanje opcije (i njenih glasova) kada `PUT`-ujete anketu sa
dodatim ili uklonjenim opcijama. To je jedini bezbedan način da se referišete na opciju - nikada njen
položaj u listi.

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
    /** Kada je postavljeno i u prošlosti, anketa je zatvorena i više ne prima glasove. **/
    closesAt?: string | null
    /** 0 anonimno (podrazumevano), 1 admini i moderatori, 2 svi. Odsustvo znači anonimno. **/
    privacy?: 0 | 1 | 2 | null
    /** Kada je true, brojevi su skriveni od svih koji još nisu glasali. Odsustvo znači false. **/
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

### Ograničenja

- Pitanje je obavezno i ima najviše 200 znakova.
- Anketa ima između 2 i 10 opcija.
- Oznaka opcije je obavezna, ima najviše 100 znakova i mora biti jedinstvena u okviru ankete (ignorišući velika/mala slova).
- `closesAt` mora biti u budućnosti prilikom kreiranja ankete. Da biste odmah zatvorili anketu, `PATCH`-ujte je
  sa datumom u prošlosti.

### Podešavanja sajta

Ankete poštuju konfiguraciju vašeg sajta, koju možete promeniti pod Customize Widget:

- Ankete moraju biti omogućene pre nego što se anketa može kreirati, inače API odgovara sa `polls-disabled`.
- Glasanje može biti ograničeno na prijavljene korisnike, u kom slučaju se glas poslat samo sa `anonUserId`
  odbija sa `poll-login-required`.