### Actions Available to All Users

- **Flag/Unflag** -- segnalare un commento per la revisione

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Block/Unblock** -- nascondere tutti i commenti di un utente (per visualizzatore)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Admin-Only Actions

- **Pin/Unpin** -- fissare un commento in cima al thread

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Lock/Unlock** -- impedire nuove risposte a un commento e bloccare modifiche ed eliminazioni finché non viene sbloccato (si applica a tutti, inclusi i moderatori)

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

Tutte le azioni di moderazione sono disponibili anche tramite il menu contestuale del commento nell'interfaccia utente. Le azioni da amministratore compaiono solo quando l'utente corrente è un amministratore del sito (impostato tramite il flag SSO `isAdmin` o la configurazione del cruscotto).