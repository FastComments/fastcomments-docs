Un `Poll` è associato a un commento, piuttosto che essere un oggetto a sé stante. Viene creato insieme al commento (vedi `POST /api/v1/comments`), o aggiunto a un commento esistente in seguito con `PUT /api/v1/polls/:commentId`.

Il conteggio dei voti è conservato direttamente sul sondaggio, quindi leggere un sondaggio fornisce i risultati senza doverli sommare. I singoli voti dietro quei conteggi sono oggetti `PollVote`.

Ogni opzione ha un `id` generato al momento della creazione del sondaggio. Quell'id è quello che usi per esprimere un voto, per rinominare un'opzione e per mantenere un'opzione (e i suoi voti) quando esegui `PUT` sul sondaggio con opzioni aggiunte o rimosse. È l'unico modo sicuro per fare riferimento a un'opzione – mai la sua posizione nella lista.

[inline-code-attrs-start title = 'Struttura del sondaggio'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

- È richiesta una domanda, e può contenere al massimo 200 caratteri.
- Un sondaggio deve avere tra 2 e 10 opzioni.
- È obbligatorio fornire un'etichetta per l'opzione, può contenere al massimo 100 caratteri e deve essere univoca all'interno del sondaggio (ignorando maiuscole/minuscole).
- `closesAt` deve essere nel futuro al momento della creazione del sondaggio. Per chiudere un sondaggio immediatamente, esegui `PATCH` con una data nel passato.

### Site Settings

I sondaggi rispettano la configurazione del tuo sito, che puoi modificare in Personalizza Widget:

- I sondaggi devono essere abilitati prima che possa essere creato un sondaggio, altrimenti l'API risponde con `polls-disabled`.
- Il voto può essere limitato agli utenti autenticati; in tal caso un voto inviato con solo un `anonUserId` viene rifiutato con `poll-login-required`.

---