### Modifica

```swift
try await sdk.editComment(commentId: commentId, newText: "Updated text")
```

Il server rigenera l'HTML. Il commento locale viene aggiornato automaticamente.

### Elimina

```swift
try await sdk.deleteComment(commentId: commentId)
```

L'eliminazione di un commento rimuove anche i suoi discendenti dall'albero locale.

Entrambe le azioni sono disponibili tramite il menu contestuale del commento nell'interfaccia utente quando l'utente corrente è l'autore del commento (o un amministratore del sito).

---
---