### Azioni Disponibili per Tutti gli Utenti

- **Segnala/Rimuovi segnalazione** -- segnala un commento per la revisione

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Blocca/Sblocca** -- nascondi tutti i commenti di un utente (per singolo visualizzatore)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Azioni Solo per Amministratori

- **Fissa/Rimuovi fissaggio** -- fissa un commento in cima alla discussione

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Blocca/Sblocca** -- impedisce nuove risposte a un commento e blocca modifiche ed eliminazioni finché non viene sbloccato (si applica a tutti, inclusi i moderatori)

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

Tutte le azioni di moderazione sono disponibili anche tramite il menu contestuale del commento nell'interfaccia utente. Le azioni da amministratore appaiono solo quando l'utente corrente è un amministratore del sito (impostato tramite il flag `isAdmin` di SSO o la configurazione della dashboard).