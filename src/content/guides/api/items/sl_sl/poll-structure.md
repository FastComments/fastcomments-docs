A `Poll` je priložena komentarju, namesto da bi bila samostojen objekt. Ustvari se skupaj s komentarjem  
(glejte `POST /api/v1/comments`), ali pa se kasneje doda obstoječemu komentarju z `PUT /api/v1/polls/:commentId`.

Števci glasov se hranijo neposredno na anketi, zato branje ankete že prikaže rezultate, ne da bi jih bilo treba seštevati. Posamezni glasovi, ki stojijo za temi števci, so objekti `PollVote`.

Vsaka možnost ima `id`, ki se ustvari ob ustvarjanju ankete. Ta id se uporablja za oddajo glasov, preimenovanje možnosti in ohranjanje možnosti (ter njenih glasov), ko anketo `PUT`-ate z dodanimi ali odstranjenimi možnostmi. To je edini varen način, da se sklicujete na možnost – nikoli na njen položaj na seznamu.

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
    /** 0 anonimno (privzeto), 1 administratorji in moderatorji, 2 vsi. Če ni nastavljeno, pomeni anonimno. **/
    privacy?: 0 | 1 | 2 | null
    /** Ko je true, so števci skriti pred vsakim, ki še ni glasoval. Če ni nastavljeno, pomeni false. **/
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

### Limits

- Vprašanje je obvezno in je dolgo največ 200 znakov.
- Anketa ima med 2 in 10 možnostmi.
- Oznaka možnosti je obvezna, je dolga največ 100 znakov in mora biti edinstvena znotraj ankete (ne glede na velikost črk).
- `closesAt` mora biti v prihodnosti, ko je anketa ustvarjena. Za takojšnjo zaprtje ankete jo `PATCH`-ajte z datumom v preteklosti.

### Site Settings

Ankete upoštevajo vašo konfiguracijo strani, ki jo lahko spremenite pod **Customize Widget**:

- Ankete morajo biti omogočene, preden je mogoče ustvariti anketo, sicer API odgovori z `polls-disabled`.
- Glasovanje je lahko omejeno na prijavljene uporabnike, v tem primeru je glas, poslan samo z `anonUserId`, zavrnjen z `poll-login-required`.