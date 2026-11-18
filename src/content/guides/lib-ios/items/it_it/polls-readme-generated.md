I sondaggi funzionano come nel widget web e si attivano dalle stesse impostazioni del widget nella tua dashboard FastComments: `pollCreation` (chi può allegare un sondaggio: disabilitato, amministratori e moderatori, o tutti) e `pollVoting` (solo utenti connessi o chiunque).

- I commenti con un sondaggio lo mostrano sotto il testo. I lettori votano toccando un'opzione, possono mostrare o nascondere i risultati prima di votare e possono cambiare il proprio voto. I conteggi si aggiornano in tempo reale mentre gli altri votano.
- I sondaggi possono chiudersi in una data, nascondere i risultati finché non voti e rendere i votanti visibili agli amministratori o a tutti ("Vedi chi ha votato").
- Quando l'utente può creare sondaggi, l'input dei commenti di `FastCommentsView` mostra un pulsante per i sondaggi che apre il generatore di sondaggi. Un sondaggio viene pubblicato insieme a un commento, quindi il testo del commento è obbligatorio. L'input della chat live omette il pulsante, come nella chat live web.

Il voto richiede un utente connesso, quindi usa [SSO](#authentication-sso-readme-generated). Il server lega un voto anonimo a un cookie di sessione, che il client API non conserva, quindi con `pollVoting` impostato su chiunque, un utente senza SSO vede "Accedi per votare". I sondaggi che nascondono i risultati finché non voti non ricevono aggiornamenti dei conteggi in tempo reale; quei votanti vedono i nuovi numeri al loro prossimo caricamento.

Per pubblicare un sondaggio dalla tua interfaccia, passa un `CommentPollInput` a `postComment`:

```swift
let draft = PollDraft(question: "Where should we eat?", options: ["Tacos", "Ramen"], closesDays: 1)
if draft.validate() == nil {
    try await sdk.postComment(text: "Lunch vote", poll: draft.toInput())
}
```

`sdk.votePoll(commentId:optionId:)` e `sdk.getPollVoters(commentId:page:search:)` coprono il voto e l'elenco dei votanti. I colori dei sondaggi sono elencati in [Poll Colors](#theming-readme-generated).