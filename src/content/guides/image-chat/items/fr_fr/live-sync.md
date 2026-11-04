### Mises à jour en temps réel

Image Chat utilise des connexions WebSocket pour synchroniser toutes les conversations en temps réel entre tous les utilisateurs connectés. Lorsqu'un utilisateur crée un nouveau marqueur, ajoute un commentaire ou supprime une discussion, tous les autres utilisateurs visualisant la même image voient la mise à jour immédiatement, sans avoir à actualiser la page.

### Fonctionnement de la synchronisation WebSocket

Lorsque vous initialisez Image Chat, le widget établit une connexion WebSocket avec les serveurs FastComments. Cette connexion reste ouverte pendant toute la durée de la session de l'utilisateur et écoute les mises à jour liées à l'image en cours.

Le système WebSocket utilise trois types de messages de diffusion pour Image Chat. L'événement `new-image-chat` se déclenche lorsqu'un utilisateur crée un nouveau marqueur sur l'image. L'événement `image-chat-updated` se déclenche lorsqu'un utilisateur met à jour une conversation existante. L'événement `deleted-image-chat` se déclenche lorsqu'un utilisateur supprime un marqueur.

### Système d'identifiant de diffusion

Pour éviter les effets d'écho où les utilisateurs verraient leurs propres actions leur être renvoyées, chaque mise à jour inclut un `broadcastId` unique. Lorsqu'un utilisateur crée ou met à jour un marqueur, son client génère un UUID pour cette opération. Lorsque le WebSocket diffuse la mise à jour à tous les clients, le client à l'origine de l'opération ignore la mise à jour car elle correspond à son propre `broadcastId`.

Cela garantit une interaction fluide où les utilisateurs voient leurs modifications immédiatement dans l'interface, sans attendre l'aller-retour via le serveur, tout en s'assurant que tous les autres utilisateurs reçoivent la mise à jour.

### Résilience de la connexion

Si la connexion WebSocket est interrompue en raison de problèmes réseau ou de maintenance du serveur, le widget tente automatiquement de se reconnecter. Pendant la période de reconnexion, les utilisateurs peuvent toujours interagir avec les marqueurs existants, mais ils ne verront pas les mises à jour en temps réel des autres utilisateurs jusqu'à ce que la connexion soit rétablie.

Une fois reconnecté, le widget se resynchronise pour s'assurer qu'aucune mise à jour n'a été manquée. Cela se fait de manière transparente, sans intervention de l'utilisateur.

### Considérations relatives à la bande passante

Les messages WebSocket sont légers et ne contiennent que les informations essentielles nécessaires à la synchronisation de l'état. La création d'un nouveau marqueur utilise généralement moins de 1 Ko de bande passante. Le système inclut également un regroupement intelligent pour réduire la fréquence des messages pendant les périodes de forte activité.

Vos métriques d'utilisation dans le tableau de bord FastComments suivent `pubSubMessageCount` et `pubSubBandwidth`, ce qui vous permet de surveiller l'activité de synchronisation en temps réel sur l'ensemble de vos sites.

### Synchronisation entre onglets

Si un utilisateur a la même page ouverte dans plusieurs onglets du navigateur, les mises à jour effectuées dans un onglet apparaissent immédiatement dans les autres onglets. Cela fonctionne via le même mécanisme de synchronisation WebSocket et ne nécessite aucune configuration supplémentaire.

Les utilisateurs peuvent avoir votre site ouvert sur plusieurs appareils simultanément, et tous resteront synchronisés. Un marqueur créé sur un ordinateur de bureau apparaît instantanément sur la tablette de l'utilisateur si les deux appareils visualisent la même image.

### Sécurité

Les messages WebSocket sont transmis via des connexions sécurisées (WSS) et incluent une validation du tenant pour garantir que les utilisateurs ne reçoivent que les mises à jour des conversations qu'ils sont autorisés à voir. Le serveur valide toutes les opérations avant de les diffuser afin de prévenir tout accès ou manipulation non autorisés.

### Comportement hors ligne

Lorsque les utilisateurs sont complètement hors ligne, ils peuvent toujours consulter les marqueurs existants, mais ne peuvent pas en créer de nouveaux ni voir les mises à jour des autres. Le widget détecte l'état hors ligne et affiche un message approprié.

Si un utilisateur tente de créer un marqueur hors ligne puis revient en ligne, l'opération échouera plutôt que d'être mise en file d'attente, garantissant ainsi la cohérence des données. Les utilisateurs doivent réessayer l'opération une fois leur connexion rétablie.

### Impact sur les performances

La connexion WebSocket a un impact minimal sur les performances. La connexion reste inactive lorsqu'aucune mise à jour n'est en cours et ne traite les messages que lorsqu'une activité se produit. Sur une image typique avec une activité de marqueurs modérée, le WebSocket utilise moins de CPU que le rendu de l'image elle-même.

Pour les pages comptant des centaines d'utilisateurs simultanés et une forte activité de création de marqueurs, le système évolue horizontalement pour maintenir les performances sans impacter les connexions des clients individuels.

### Cas d'usage collaboratifs

La synchronisation en temps réel rend Image Chat particulièrement puissant pour les flux de travail collaboratifs. Les équipes de design peuvent examiner des maquettes ensemble, chacun voyant le placement des marqueurs en temps réel. Les équipes de support client peuvent annoter collaborativement des captures d'écran pour identifier les problèmes. Les groupes éducatifs peuvent discuter de diagrammes, tous les participants voyant les marqueurs des autres au fur et à mesure de leur création.

Le retour immédiat crée une expérience collaborative plus engageante et plus productive par rapport aux systèmes de commentaires traditionnels où les utilisateurs doivent actualiser la page pour voir les mises à jour.