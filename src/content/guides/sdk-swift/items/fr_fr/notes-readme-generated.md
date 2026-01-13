### Identifiants de diffusion

Vous verrez qu'on vous demande de passer un `broadcastId` dans certains appels d'API. Lorsque vous recevez des événements, vous récupérerez cet ID en retour, ce qui vous permettra d'ignorer l'événement si vous prévoyez d'appliquer des changements de manière optimiste côté client (ce que vous souhaiterez probablement faire, car cela offre la meilleure expérience). Passez un UUID ici. L'ID doit être suffisamment unique pour ne pas apparaître deux fois au cours d'une session.

```swift
let broadcastId = UUID().uuidString
```