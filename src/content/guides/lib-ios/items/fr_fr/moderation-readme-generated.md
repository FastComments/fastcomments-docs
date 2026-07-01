### Actions Available to All Users

- **Flag/Unflag** -- signaler un commentaire pour révision

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Block/Unblock** -- masquer tous les commentaires d'un utilisateur (par visualiseur)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Admin-Only Actions

- **Pin/Unpin** -- épingler un commentaire en haut du fil

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Lock/Unlock** -- empêcher les nouvelles réponses à un commentaire, et bloquer les modifications et suppressions jusqu'à ce qu'il soit déverrouillé (s'applique à tous, y compris les modérateurs)

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

Toutes les actions de modération sont également disponibles via le menu contextuel du commentaire dans l'interface utilisateur. Les actions d'administrateur n'apparaissent que lorsque l'utilisateur actuel est un administrateur du site (défini via le drapeau SSO `isAdmin` ou la configuration du tableau de bord).