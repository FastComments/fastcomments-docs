Un `Poll` è collegato a un commento, anziché essere un oggetto a sé stante. Viene creato insieme al commento
(vedi `POST /api/v1/comments`), oppure aggiunto successivamente a un commento esistente con `PUT /api/v1/polls/:commentId`.

I conteggi dei voti sono conservati direttamente sul sondaggio, quindi la lettura di un sondaggio fornisce i risultati
senza dover sommare nulla. I singoli voti alla base di quei conteggi sono oggetti `PollVote`.

Ogni opzione ha un `id` generato quando il sondaggio viene creato. Quell'id è ciò che usi per esprimere un voto,
per rinominare un'opzione e per mantenere un'opzione (e i suoi voti) quando esegui un `PUT` del sondaggio con opzioni
aggiunte o rimosse. È l'unico modo sicuro per riferirsi a un'opzione - mai alla sua posizione nell'elenco.

[inline-code-attrs-start title = 'Struttura del Poll'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** Se impostato e nel passato, il sondaggio è chiuso e non accetta più voti. **/
    closesAt?: string | null
    /** 0 anonimo (predefinito), 1 amministratori e moderatori, 2 tutti. Assente significa anonimo. **/
    privacy?: 0 | 1 | 2 | null
    /** Se true, i conteggi sono nascosti a chiunque non abbia ancora votato. Assente significa false. **/
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

### Limiti

- Una domanda è obbligatoria e ha al massimo 200 caratteri.
- Un sondaggio ha tra 2 e 10 opzioni.
- L'etichetta di un'opzione è obbligatoria, ha al massimo 100 caratteri e deve essere unica all'interno del sondaggio (ignorando maiuscole/minuscole).
- `closesAt` deve essere nel futuro quando il sondaggio viene creato. Per chiudere immediatamente un sondaggio, esegui un `PATCH` con
  una data nel passato.

### Impostazioni del Sito

I sondaggi rispettano la configurazione del tuo sito, che puoi modificare sotto Customize Widget:

- I sondaggi devono essere abilitati prima che sia possibile crearne uno, altrimenti l'API risponde con `polls-disabled`.
- Il voto può essere limitato agli utenti registrati, nel qual caso un voto inviato con solo un `anonUserId` viene rifiutato
  con `poll-login-required`.