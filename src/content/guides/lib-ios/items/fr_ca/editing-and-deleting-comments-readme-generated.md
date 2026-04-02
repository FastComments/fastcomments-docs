### Modifier

```swift
try await sdk.editComment(commentId: commentId, newText: "Updated text")
```

Le serveur réexécute le rendu du HTML. Le commentaire local se met à jour automatiquement.

### Supprimer

```swift
try await sdk.deleteComment(commentId: commentId)
```

La suppression d'un commentaire supprime également ses descendants de l'arbre local.

Les deux actions sont disponibles via le menu contextuel du commentaire dans l'interface utilisateur lorsque l'utilisateur actuel est l'auteur du commentaire (ou un administrateur du site).