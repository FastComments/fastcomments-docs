`Poll` je pripet komentarju, namesto da bi bil samostojen objekt. Ustvari se skupaj s komentarjem
(glej `POST /api/v1/comments`) ali pa se kasneje doda obstoječemu komentarju z `PUT /api/v1/polls/:commentId`.

Števci glasov se hranijo na sami anketi, zato branje ankete da rezultate brez seštevanja.
Posamezni glasovi za temi števci so objekti `PollVote`.

Vsaka možnost ima `id`, ki se generira ob ustvarjanju ankete. Ta id uporabite za oddajo glasu,
za preimenovanje možnosti in za ohranitev možnosti (in njenih glasov), ko anketo `PUT`-ate z dodanimi
ali odstranjenimi možnostmi. To je edini varen način sklicevanja na možnost - nikoli njen položaj na seznamu.

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
    /** Ko je nastavljeno in v preteklosti, je anketa zaprta in ne sprejema več glasov. **/
    closesAt?: string | null
    /** 0 anonimno (privzeto), 1 administratorji in moderatorji, 2 vsi. Odsotnost pomeni anonimno. **/
    privacy?: 0 | 1 | 2 | null
    /** Ko je true, so števci skriti vsem, ki še niso glasovali. Odsotnost pomeni false. **/
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

### Omejitve

- Vprašanje je obvezno in ima največ 200 znakov.
- Anketa ima od 2 do 10 možnosti.
- Oznaka možnosti je obvezna, ima največ 100 znakov in mora biti znotraj ankete edinstvena (ne glede na velikost črk).
- `closesAt` mora biti v prihodnosti ob ustvarjanju ankete. Za takojšnje zaprtje ankete jo `PATCH`-ajte z
  datumom v preteklosti.

### Nastavitve strani

Ankete upoštevajo konfiguracijo vaše strani, ki jo lahko spremenite pod Customize Widget:

- Ankete morajo biti omogočene, preden lahko ustvarite anketo, sicer API odgovori z `polls-disabled`.
- Glasovanje je lahko omejeno na prijavljene uporabnike, v tem primeru je glas, poslan samo z `anonUserId`, zavrnjen
  z `poll-login-required`.