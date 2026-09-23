A `PollVote` je odgovor jedne osobe na anketu. Brojke prikazane na samoj anketi ažurirane su u skladu s njima, pa ih trebate samo kada želite znati *tko* je glasao za što, a ne ukupne zbrojeve.

Glasatelj može imati najviše jedan glas po anketi. Ponovno glasanje premješta njihov postojeći glas na novu opciju umjesto da doda drugi, a `updatedAt` bilježi kada se to dogodilo.

`voterId` je `userId` kada je glasatelj bio prijavljen, a `anonUserId` u suprotnom.

[inline-code-attrs-start title = 'Struktura PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVote {
    id: string
    tenantId: string
    commentId: string
    urlId: string
    /** The userId when the voter was logged in, otherwise the anonUserId. **/
    voterId: string
    optionId: string
    createdAt: string
    /** When the voter last moved their vote to a different option. **/
    updatedAt?: string
}
[inline-code-end]

### Privatnost

Postavka `privacy` ankete primjenjuje se na ovaj API na isti način kao što se primjenjuje u widgetu za komentare:

- **Anonymous** (zadano): nitko ne može vidjeti kako je netko glasao, pa se glasovi ne mogu pročitati.  
  `GET /api/v1/poll-votes` i `GET /api/v1/poll-votes/:id` odgovaraju s `poll-anonymous`. Brojke ankete i dalje su dostupne putem `GET /api/v1/polls/:commentId`.

- **Admins and moderators**: vaš API ključ pripada administratoru vaše stranice, pa može čitati glasove.

- **Everyone**: glasovi se mogu čitati.

Privatnost ankete može se suziti, ali ne i proširiti nakon što ima glasova.

---