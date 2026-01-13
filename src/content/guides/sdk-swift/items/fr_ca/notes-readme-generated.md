### Identifiants de diffusion

Vous verrez que vous devez passer un `broadcastId` dans certaines requêtes d'API. Lorsque vous recevez des événements, vous récupérerez cet ID, ce qui vous permet de savoir qu'il faut ignorer l'événement si vous prévoyez d'appliquer des changements de façon optimiste côté client (ce que vous voudrez probablement faire, car cela offre la meilleure expérience). Passez un UUID ici. L'ID doit être suffisamment unique pour ne pas apparaître deux fois au cours d'une même session.

```swift
let broadcastId = UUID().uuidString
```