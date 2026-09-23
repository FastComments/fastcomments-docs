A `PollVote` je odgovor jedne osobe na anketu. Brojke prikazane na samoj anketi ažurirane su u skladu s ovim, pa su vam potrebne samo kada želite da znate *ko* je glasao za šta, a ne ukupne rezultate.

Glasač može imati najviše jedan glas po anketi. Ponovno glasanje premješta njihov postojeći glas na novu opciju umesto da doda drugi, a `updatedAt` beleži kada se to dogodilo.

`voterId` je `userId` kada je glasač bio prijavljen, a `anonUserId` u suprotnom.

[inline-code-attrs-start title = 'Struktura PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVote {
    id: string
    tenantId: string
    commentId: string
    urlId: string
    /** userId kada je glasač bio prijavljen, u suprotnom anonUserId. **/
    voterId: string
    optionId: string
    createdAt: string
    /** Kada je glasač poslednji put premestio svoj glas na drugu opciju. **/
    updatedAt?: string
}
[inline-code-end]

### Privatnost

- **Anoniman** (podrazumevano): niko ne može videti kako je ko glasao, pa glasovi ne mogu biti pročitani.
  `GET /api/v1/poll-votes` i `GET /api/v1/poll-votes/:id` odgovaraju sa `poll-anonymous`. Brojke ankete
  i dalje su dostupne putem `GET /api/v1/polls/:commentId`.
- **Administratori i moderatori**: vaš API ključ pripada administratoru vašeg sajta, pa može čitati glasove.
- **Svi**: glasovi se mogu čitati.

Privatnost ankete može biti sužena, ali ne i proširena nakon što ima glasova.