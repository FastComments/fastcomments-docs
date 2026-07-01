### Actions Available to All Users

- **Flag/Unflag** -- signaler un commentaire pour examen

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Block/Unblock** -- masquer tous les commentaires d’un utilisateur (par visionneur)

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

- **Lock/Unlock** -- empêcher les nouvelles réponses à un commentaire, et bloquer les modifications et suppressions jusqu’à ce qu’il soit déverrouillé (s’applique à tout le monde, y compris les modérateurs)

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

Toutes les actions de modération sont également disponibles via le menu contextuel du commentaire dans l’UI. Les actions d’administration n’apparaissent que lorsque l’utilisateur actuel est un administrateur du site (défini via le drapeau `isAdmin` SSO ou la configuration du tableau de bord).