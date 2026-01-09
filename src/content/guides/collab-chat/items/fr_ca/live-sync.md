### Real-Time Updates

Collab Chat utilise des connexions WebSocket pour synchroniser toutes les conversations en temps réel entre tous les utilisateurs connectés. Lorsqu'une personne crée une nouvelle annotation, ajoute un commentaire ou supprime une discussion, tous les autres utilisateurs qui consultent la même page voient la mise à jour immédiatement sans actualiser.

### How WebSocket Sync Works

Lorsque vous initialisez Collab Chat, le widget établit une connexion WebSocket avec les serveurs FastComments. Cette connexion reste ouverte pendant la durée de la session de l'utilisateur et écoute les mises à jour liées à la page courante.

Le système WebSocket utilise trois types de messages de diffusion pour Collab Chat. L'événement `new-text-chat` se déclenche lorsqu'une personne crée une nouvelle annotation sur la page. L'événement `updated-text-chat` se déclenche lorsqu'une personne met à jour une conversation existante. L'événement `deleted-text-chat` se déclenche lorsqu'une personne supprime une annotation.

### Broadcast ID System

Pour éviter les effets d'écho où les utilisateurs voient leurs propres actions leur être renvoyées, chaque mise à jour comprend un `broadcastId` unique. Lorsqu'un utilisateur crée ou met à jour une annotation, son client génère un UUID pour cette opération. Lorsque le WebSocket diffuse la mise à jour à tous les clients, le client d'origine ignore la mise à jour parce qu'elle correspond à son propre `broadcastId`.

Cela garantit une interaction fluide où les utilisateurs voient leurs modifications immédiatement dans l'UI sans attendre l'aller-retour via le serveur, tout en s'assurant que tous les autres utilisateurs reçoivent la mise à jour.

### Live User Count

La barre supérieure affiche le nombre d'utilisateurs qui consultent actuellement la page. Ce compteur se met à jour en temps réel au fur et à mesure que les utilisateurs rejoignent et quittent. Le nombre d'utilisateurs est fourni via la même connexion WebSocket et s'incrémente/se décrémente automatiquement en fonction des événements de connexion et de déconnexion.

### Connection Resilience

Si la connexion WebSocket tombe en raison de problèmes réseau ou de maintenance du serveur, le widget tente automatiquement de se reconnecter. Pendant la période de reconnexion, les utilisateurs peuvent toujours interagir avec les annotations existantes, mais ils ne verront pas les mises à jour en temps réel des autres utilisateurs tant que la connexion n'est pas rétablie.

Une fois reconnecté, le widget se resynchronise pour s'assurer qu'aucune mise à jour n'a été manquée. Cela se fait de manière transparente sans nécessiter l'intervention de l'utilisateur.

### Bandwidth Considerations

Les messages WebSocket sont légers et ne contiennent que les informations essentielles nécessaires pour synchroniser l'état. La création d'une nouvelle annotation utilise généralement moins de 1 Ko de bande passante. Le système inclut également un regroupement intelligent pour réduire la fréquence des messages lors des périodes d'activité élevée.

Vos métriques d'utilisation dans le tableau de bord FastComments suivent `pubSubMessageCount` et `pubSubBandwidth` afin que vous puissiez surveiller l'activité de synchronisation en temps réel sur vos sites.

### Cross-Tab Synchronization

Si un utilisateur a la même page ouverte dans plusieurs onglets du navigateur, les mises à jour d'un onglet apparaissent immédiatement dans les autres onglets. Cela fonctionne via le même mécanisme de synchronisation WebSocket et ne nécessite aucune configuration supplémentaire.

### Security

Les messages WebSocket sont transmis via des connexions sécurisées (WSS) et incluent une validation du tenant pour s'assurer que les utilisateurs ne reçoivent que les mises à jour des conversations qu'ils sont autorisés à voir. Le serveur valide toutes les opérations avant de les diffuser afin d'empêcher tout accès ou manipulation non autorisé.