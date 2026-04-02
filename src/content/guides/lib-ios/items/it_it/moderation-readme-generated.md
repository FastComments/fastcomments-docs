---
### Azioni disponibili per tutti gli utenti

- **Segnala/Rimuovi segnalazione** -- segnala un commento per la revisione

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Blocca/Sblocca** -- nascondi tutti i commenti di un utente (per visualizzatore)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Azioni riservate agli amministratori

- **Fissa/Rimuovi fissaggio** -- fissa un commento in cima alla discussione

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Blocca/Sblocca** -- impedisci nuove risposte a un commento

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

Tutte le azioni di moderazione sono disponibili anche tramite il menu contestuale del commento nell'interfaccia utente. Le azioni da amministratore compaiono solo quando l'utente corrente è un amministratore del sito (impostato tramite il flag SSO `isAdmin` o la configurazione della dashboard).

---
---