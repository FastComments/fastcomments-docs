Un `PollVote` è la risposta di una persona a un sondaggio. I conteggi mostrati sul sondaggio stesso sono tenuti aggiornati con questi, quindi li servono solo quando vuoi sapere *chi* ha votato per cosa, piuttosto che i totali.

Un elettore può avere al massimo un voto per sondaggio. Votare di nuovo sposta il voto esistente verso la nuova opzione invece di aggiungerne un secondo, e `updatedAt` registra quando ciò è avvenuto.

`voterId` è il `userId` quando l'elettore era connesso, altrimenti è l'`anonUserId`.

[inline-code-attrs-start title = 'Struttura PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVote {
    id: string
    tenantId: string
    commentId: string
    urlId: string
    /** L'userId quando l'elettore era connesso, altrimenti l'anonUserId. **/
    voterId: string
    optionId: string
    createdAt: string
    /** Quando l'elettore ha spostato per l'ultima volta il suo voto a un'opzione diversa. **/
    updatedAt?: string
}
[inline-code-end]

### Privacy

L'impostazione `privacy` del sondaggio si applica a questa API nello stesso modo in cui si applica nel widget dei commenti:

- **Anonymous** (il valore predefinito): nessuno può vedere come qualcuno ha votato, quindi i voti non possono essere letti.
  `GET /api/v1/poll-votes` e `GET /api/v1/poll-votes/:id` rispondono con `poll-anonymous`. I conteggi del sondaggio
  sono ancora disponibili da `GET /api/v1/polls/:commentId`.
- **Admins and moderators**: la tua chiave API appartiene all'amministratore del tuo sito, quindi può leggere i voti.
- **Everyone**: i voti possono essere letti.

La privacy del sondaggio può essere ridotta ma non ampliata una volta che ha dei voti.