### Mises à jour en temps réel

Collab Chat utilise des connexions WebSocket pour synchroniser toutes les conversations en temps réel entre tous les utilisateurs connectés. Lorsqu'une personne crée une nouvelle annotation, ajoute un commentaire ou supprime une discussion, tous les autres utilisateurs affichant la même page voient la mise à jour immédiatement sans actualiser.

### Comment fonctionne la synchronisation WebSocket

Lorsque vous initialisez Collab Chat, le widget établit une connexion WebSocket aux serveurs FastComments. Cette connexion reste ouverte pendant la durée de la session de l'utilisateur et écoute les mises à jour liées à la page en cours.

Le système WebSocket utilise trois types de messages de diffusion pour Collab Chat. L'événement `new-text-chat` se déclenche lorsqu'une personne crée une nouvelle annotation sur la page. L'événement `updated-text-chat` se déclenche lorsqu'une personne met à jour une conversation existante. L'événement `deleted-text-chat` se déclenche lorsqu'une personne supprime une annotation.

### Système d'ID de diffusion

Pour éviter les effets d'écho où les utilisateurs voient leurs propres actions leur être renvoyées, chaque mise à jour inclut un `broadcastId` unique. Lorsqu'un utilisateur crée ou met à jour une annotation, son client génère un UUID pour cette opération. Lorsque le WebSocket diffuse la mise à jour à tous les clients, le client d'origine ignore la mise à jour parce qu'elle correspond à son propre `broadcastId`.

Cela garantit une interaction fluide où les utilisateurs voient leurs modifications immédiatement dans l'UI sans attendre l'aller-retour via le serveur, tout en s'assurant que tous les autres utilisateurs reçoivent la mise à jour.

### Nombre d'utilisateurs connectés

La barre supérieure affiche le nombre d'utilisateurs actuellement en train de consulter la page. Ce nombre se met à jour en temps réel au fur et à mesure que des utilisateurs rejoignent et quittent. Le nombre d'utilisateurs est fourni via la même connexion WebSocket et s'incrémente/s décrémente automatiquement en fonction des événements de connexion et de déconnexion.

### Résilience de la connexion

Si la connexion WebSocket tombe à cause de problèmes réseau ou de maintenance serveur, le widget tente automatiquement de se reconnecter. Pendant la période de reconnexion, les utilisateurs peuvent toujours interagir avec les annotations existantes, mais ils ne verront pas les mises à jour en temps réel des autres utilisateurs tant que la connexion n'est pas rétablie.

Une fois reconnecté, le widget se resynchronise pour s'assurer qu'aucune mise à jour n'a été manquée. Cela se produit de manière transparente sans nécessiter d'intervention de l'utilisateur.

### Considérations sur la bande passante

Les messages WebSocket sont légers et ne contiennent que les informations essentielles nécessaires pour synchroniser l'état. La création d'une nouvelle annotation utilise généralement moins de 1 Ko de bande passante. Le système inclut également un regroupement intelligent pour réduire la fréquence des messages pendant les périodes d'activité élevée.

Vos métriques d'utilisation dans le tableau de bord FastComments suivent `pubSubMessageCount` et `pubSubBandwidth` afin que vous puissiez surveiller l'activité de synchronisation en temps réel sur vos sites.

### Synchronisation entre onglets

Si un utilisateur a la même page ouverte dans plusieurs onglets du navigateur, les mises à jour dans un onglet apparaissent immédiatement dans les autres onglets. Cela fonctionne via le même mécanisme de synchronisation WebSocket et ne nécessite aucune configuration supplémentaire.

### Sécurité

Les messages WebSocket sont transmis via des connexions sécurisées (WSS) et incluent la validation du tenant pour garantir que les utilisateurs ne reçoivent que les mises à jour des conversations qu'ils sont autorisés à voir. Le serveur valide toutes les opérations avant de les diffuser afin d'empêcher tout accès ou manipulation non autorisé.