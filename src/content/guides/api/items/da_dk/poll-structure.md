A `Poll` er knyttet til en kommentar i stedet for at være et selvstændigt objekt. Den oprettes sammen med kommentaren  
(se `POST /api/v1/comments`), eller tilføjes til en eksisterende kommentar senere med `PUT /api/v1/polls/:commentId`.

Stemmetællerne gemmes på selve afstemningen, så læsning af en afstemning giver dig resultaterne uden at skulle lægge noget sammen. De individuelle stemmer bag disse tællere er `PollVote`‑objekter.

Hver mulighed har en `id`, som genereres, når afstemningen oprettes. Det er den id, du bruger til at afgive en stemme, til at omdøbe en mulighed, og til at beholde en mulighed (og dens stemmer), når du `PUT`‑er afstemningen med tilføjede eller fjernede muligheder. Det er den eneste sikre måde at referere til en mulighed på – aldrig dens position i listen.

[inline-code-attrs-start title = 'Afstemningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Begrænsninger

- Et spørgsmål er påkrævet, og må højst være 200 tegn.  
- En afstemning har mellem 2 og 10 muligheder.  
- En valgmærkat er påkrævet, må højst være 100 tegn, og skal være unik inden for afstemningen (ignorerer store/små bogstaver).  
- `closesAt` skal være i fremtiden, når afstemningen oprettes. For at lukke en afstemning med det samme, `PATCH` den med en dato i fortiden.

### Webstedsindstillinger

Afstemninger følger din webstedskonfiguration, som du kan ændre under Tilpas Widget:

- Afstemninger skal være aktiveret, før en afstemning kan oprettes, ellers svarer API'en med `polls-disabled`.  
- Afstemning kan begrænses til loggede brugere, i så fald afvises en stemme sendt kun med en `anonUserId` med `poll-login-required`.