Abonnez-vous aux événements en direct pour recevoir des mises à jour en temps réel sur les commentaires, les votes et d'autres activités.

### Événements au niveau de la page

Écoutez les événements en direct sur une page spécifique (commentaires, votes, etc.) :

```typescript
import { subscribeToChanges, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const config = {
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id',
};

// S'abonner aux événements en direct pour une page
const subscription = subscribeToChanges(
  config,
  'your-tenant-id', // tenantIdWS
  'page-url-id',    // urlIdWS  
  'user-session-id', // userIdWS (obtenez ceci à partir de la réponse de getComments)
  (event: LiveEvent) => {
    console.log('Live event received:', event);
    
    switch (event.type) {
      case LiveEventType.new_comment:
        console.log('New comment:', event.comment);
        // Mettez à jour votre interface utilisateur avec le nouveau commentaire
        break;
      case LiveEventType.new_vote:
        console.log('New vote:', event.vote);
        // Mettez à jour les compteurs de votes dans votre interface utilisateur
        break;
      case LiveEventType.updated_comment:
        console.log('Comment updated:', event.comment);
        break;
      default:
        console.log('Other event type:', event.type);
    }
    
    return true; // Retournez true si l'événement a été traité
  },
  (isConnected: boolean) => {
    console.log('Connection status:', isConnected ? 'Connected' : 'Disconnected');
  }
);

// Fermez l'abonnement lorsque terminé
subscription.close();
```

### S'abonner aux événements utilisateur

Écoutez les événements spécifiques à un utilisateur (notifications, mentions, etc.) :

```typescript
import { subscribeToUserFeed, LiveEvent, LiveEventType } from 'fastcomments-sdk/browser';

const userConfig = {
  userIdWS: 'user-session-id', // Obtenez ceci à partir de la réponse de getComments
};

// Abonnez-vous au fil personnel de l'utilisateur
const userSubscription = subscribeToUserFeed(
  userConfig,
  (event: LiveEvent) => {
    console.log('User event received:', event);
    
    switch (event.type) {
      case LiveEventType.notification:
        console.log('New notification:', event.notification);
        // Affichez la notification dans votre interface utilisateur
        break;
      case LiveEventType.notification_update:
        console.log('Notification updated:', event.notification);
        break;
      default:
        console.log('Other user event:', event.type);
    }
    
    return true;
  },
  (isConnected: boolean) => {
    console.log('User feed connection:', isConnected ? 'Connected' : 'Disconnected');
  }
);

// Fermez lorsque terminé
userSubscription.close();
```

### Obtenir userIdWS

Le paramètre `userIdWS` est requis pour les événements en direct et peut être obtenu à partir des réponses de l'API :

```typescript
const response = await sdk.publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-id'
});

// Extraire userIdWS de la réponse
const userIdWS = response.data?.userSessionInfo?.userIdWS;

if (userIdWS) {
  // Vous pouvez maintenant vous abonner aux événements en direct
  const subscription = subscribeToChanges(config, tenantIdWS, urlIdWS, userIdWS, handleEvent);
}
```