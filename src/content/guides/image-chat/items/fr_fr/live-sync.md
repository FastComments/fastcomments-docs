### Real-Time Updates

Image Chat utilise des connexions WebSocket pour synchroniser toutes les conversations en temps réel entre tous les utilisateurs connectés. Lorsqu'une personne crée un nouveau marqueur, ajoute un commentaire ou supprime une discussion, tous les autres utilisateurs visualisant la même image voient la mise à jour immédiatement sans recharger.

### How WebSocket Sync Works

Lorsque vous initialisez Image Chat, le widget établit une connexion WebSocket avec les serveurs FastComments. Cette connexion reste ouverte pendant la durée de la session de l'utilisateur et écoute les mises à jour liées à l'image en cours.

Le système WebSocket utilise trois types de messages de diffusion pour Image Chat. L'événement `new-image-chat` se déclenche lorsqu'une personne crée un nouveau marqueur sur l'image. L'événement `image-chat-updated` se déclenche lorsqu'une personne met à jour une conversation existante. L'événement `deleted-image-chat` se déclenche lorsqu'une personne supprime un marqueur.

### Broadcast ID System

Pour éviter les effets d'écho où les utilisateurs voient leurs propres actions leur être renvoyées, chaque mise à jour inclut un `broadcastId` unique. Lorsqu'un utilisateur crée ou met à jour un marqueur, son client génère un UUID pour cette opération. Lorsque le WebSocket diffuse la mise à jour à tous les clients, le client d'origine ignore la mise à jour parce qu'elle correspond à son propre `broadcastId`.

Cela garantit une interaction fluide où les utilisateurs voient leurs modifications immédiatement dans l'interface utilisateur sans attendre l'aller-retour via le serveur, tout en assurant que tous les autres utilisateurs reçoivent la mise à jour.

### Connection Resilience

Si la connexion WebSocket est interrompue en raison de problèmes réseau ou de maintenance serveur, le widget tente automatiquement de se reconnecter. Pendant la période de reconnexion, les utilisateurs peuvent toujours interagir avec les marqueurs existants, mais ils ne verront pas les mises à jour en temps réel des autres utilisateurs tant que la connexion n'est pas rétablie.

Une fois reconnecté, le widget resynchronise pour s'assurer qu'aucune mise à jour n'a été manquée. Cela se produit de manière transparente sans nécessiter l'intervention de l'utilisateur.

### Bandwidth Considerations

Les messages WebSocket sont légers et contiennent uniquement les informations essentielles nécessaires pour synchroniser l'état. La création d'un nouveau marqueur utilise généralement moins de 1 Ko de bande passante. Le système inclut également un regroupement intelligent pour réduire la fréquence des messages pendant les périodes d'activité élevée.

Vos métriques d'utilisation dans le tableau de bord FastComments suivent `pubSubMessageCount` et `pubSubBandwidth` afin que vous puissiez surveiller l'activité de synchronisation en temps réel sur vos sites.

### Cross-Tab Synchronization

Si un utilisateur a la même page ouverte dans plusieurs onglets du navigateur, les mises à jour dans un onglet apparaissent immédiatement dans les autres onglets. Cela fonctionne via le même mécanisme de synchronisation WebSocket et ne nécessite aucune configuration supplémentaire.

Les utilisateurs peuvent avoir votre site ouvert sur plusieurs appareils simultanément, et tous resteront synchronisés. Un marqueur créé sur un ordinateur de bureau apparaît instantanément sur la tablette de l'utilisateur si les deux appareils affichent la même image.

### Security

Les messages WebSocket sont transmis sur des connexions sécurisées (WSS) et incluent une validation du locataire pour garantir que les utilisateurs ne reçoivent que les mises à jour des conversations qu'ils sont autorisés à voir. Le serveur valide toutes les opérations avant de les diffuser afin d'empêcher tout accès ou manipulation non autorisée.

### Offline Behavior

Lorsque les utilisateurs sont complètement hors ligne, ils peuvent toujours consulter les marqueurs existants mais ne peuvent pas en créer de nouveaux ni voir les mises à jour des autres. Le widget détecte l'état hors ligne et affiche un message approprié.

Si un utilisateur tente de créer un marqueur en étant hors ligne puis revient en ligne, l'opération échouera plutôt que d'être mise en file d'attente, garantissant la cohérence des données. Les utilisateurs doivent réessayer l'opération une fois leur connexion rétablie.

### Performance Impact

La connexion WebSocket a un impact minimal sur les performances. La connexion reste en veille lorsqu'aucune mise à jour n'a lieu et ne traite les messages que lorsqu'il y a de l'activité. Sur une image typique avec une activité modérée de marqueurs, le WebSocket utilise moins de CPU que le rendu de l'image lui‑même.

Pour les pages avec des centaines d'utilisateurs simultanés et une forte création de marqueurs, le système s'étend horizontalement pour maintenir les performances sans impacter les connexions des clients individuels.

### Collaborative Use Cases

La synchronisation en temps réel rend Image Chat particulièrement puissant pour les flux de travail collaboratifs. Les équipes de conception peuvent revoir des maquettes ensemble en voyant tous les emplacements de marqueurs en temps réel. Les équipes d'assistance client peuvent annoter des captures d'écran de manière collaborative pour identifier des problèmes. Les groupes éducatifs peuvent discuter de diagrammes avec tous les participants voyant les marqueurs des autres au fur et à mesure de leur création.

Le retour immédiat crée une expérience collaborative plus engageante et productive comparée aux systèmes de commentaires traditionnels où les utilisateurs doivent rafraîchir pour voir les mises à jour.