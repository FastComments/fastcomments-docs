A `PollVote` er én persons svar på en afstemning. Tællingerne, der vises på afstemningen selv, holdes i takt med disse, så du kun har brug for dem, når du vil vide *hvem* der stemte på hvad, snarere end totalerne.

En vælger har højst én stemme pr. afstemning. At stemme igen flytter deres eksisterende stemme til den nye mulighed i stedet for at tilføje en anden, og `updatedAt` registrerer, hvornår det skete.

`voterId` er `userId` når vælgeren var logget ind, og ellers `anonUserId`.

[inline-code-attrs-start title = 'PollVote Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Privacy

Afstemningens `privacy`-indstilling gælder for dette API på samme måde som den gælder i kommentarfunktionen:

- **Anonymous** (standardindstillingen): ingen kan se, hvordan nogen stemte, så stemmerne kan ikke læses.
  `GET /api/v1/poll-votes` og `GET /api/v1/poll-votes/:id` svarer med `poll-anonymous`. Afstemningens tællinger er stadig tilgængelige fra `GET /api/v1/polls/:commentId`.
- **Admins and moderators**: din API-nøgle tilhører din sides administrator, så den kan læse stemmerne.
- **Everyone**: stemmerne kan læses.

Afstemningens privatliv kan indsnævres, men ikke udvides, når den har stemmer.

---