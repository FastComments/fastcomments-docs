### Actions disponibles pour tous les utilisateurs

- **Signaler/Annuler le signalement** -- signaler un commentaire pour examen

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Bloquer/Débloquer** -- masquer tous les commentaires d'un utilisateur (par spectateur)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Actions réservées aux administrateurs

- **Épingler/Désépingler** -- épingler un commentaire en haut du fil de discussion

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Verrouiller/Déverrouiller** -- empêcher les nouvelles réponses à un commentaire, et bloquer les modifications et suppressions jusqu'à son déverrouillage (s'applique à tout le monde, y compris les modérateurs)

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

Toutes les actions de modération sont également disponibles via le menu contextuel du commentaire dans l'interface. Les actions d'administration n'apparaissent que lorsque l'utilisateur actuel est un administrateur du site (défini via le flag `isAdmin` du SSO ou la configuration du tableau de bord).

---
---