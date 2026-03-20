La Messagerie directe (DM) permet aux utilisateurs de FastComments d'avoir des conversations privées en tête-à-tête, séparées des commentaires publics et des interactions de profil.

### Qu'est-ce que la Messagerie directe ?

La Messagerie directe offre :
- **Conversations privées** entre deux utilisateurs
- **Messagerie en temps réel** avec notifications
- **Historique des conversations** persistant entre les sessions
- **Indicateurs de statut en ligne** pour voir quand quelqu'un est disponible
- **Suivi des messages non lus** pour rester au courant des nouveaux messages

### Démarrer un message direct

Il existe plusieurs façons d'initier une conversation DM :

**Depuis un profil utilisateur :**
1. Visitez la page de profil de l'utilisateur
2. Cliquez sur l'onglet "Messages directs" ou le bouton "Envoyer un message"
3. Tapez votre message et envoyez

**Depuis un commentaire :**
Certaines implémentations permettent de cliquer sur le nom ou l'avatar d'un utilisateur pour accéder rapidement à son profil et aux options de messagerie.

**Lien direct :**
Si vous avez un ID de conversation ou un lien de redirection DM, vous pouvez accéder directement aux conversations.

### Accéder à vos messages

Pour voir toutes vos conversations de messagerie directe :

1. Visitez votre page de profil
2. Cliquez sur l'onglet "Messages directs"
3. Voyez la liste de toutes vos conversations

Chaque conversation affiche :
- L'avatar et le nom de l'autre participant
- Leur statut en ligne/hors ligne (indicateur vert lorsqu'il est en ligne)
- Un aperçu du dernier message
- Le nombre de messages non lus (le cas échéant)
- L'horodatage de la dernière activité

### Vue de la conversation

Lorsque vous ouvrez une conversation, vous verrez :

- **Historique complet des messages** - Tous les messages entre vous et l'autre utilisateur
- **Mises à jour en temps réel** - Les nouveaux messages apparaissent instantanément via WebSocket
- **Statut en ligne** - Voir si l'autre personne est actuellement en ligne
- **Horodatages des messages** - Quand chaque message a été envoyé
- **Composition du message** - Zone de texte pour taper et envoyer de nouveaux messages

### Notifications de message

Vous recevrez des notifications pour les nouveaux messages directs :

- **Notifications dans l'application** - Compteur de badge sur votre profil
- **Notifications par courriel** - Selon vos paramètres de notification
- **Alertes en temps réel** - Notifications instantanées lorsque vous êtes connecté

Gérez les préférences de notification dans vos [Paramètres du compte](https://fastcomments.com/auth/my-account/edit-notifications).

### Confidentialité et blocage

**Désactiver les messages directs :**
Si vous ne souhaitez pas recevoir de DM, vous pouvez les désactiver complètement :
1. Allez dans les Paramètres de confidentialité sur votre profil
2. Activez "Désactiver les messages directs"
3. L'option de DM sera cachée sur votre profil

Pour plus de détails, voir [Paramètres de confidentialité](/guide-user-profiles.html#privacy-settings).

**Bloquer des utilisateurs :**
Si quelqu'un vous harcèle via DM :
1. Visitez son profil
2. Bloquez l'utilisateur
3. Il ne pourra plus vous envoyer de messages

Les utilisateurs bloqués ne peuvent pas :
- Vous envoyer de messages directs
- Voir votre activité
- Commenter votre profil
- Interagir avec vous n'importe où sur FastComments

### Gérer les conversations

**Masquer des conversations :**
Vous pouvez masquer les conversations que vous ne voulez plus voir dans votre liste de conversations :
1. Ouvrez votre onglet Messages directs
2. Trouvez la conversation à masquer
3. Sélectionnez l'option masquer/archiver

Les conversations masquées n'apparaîtront pas dans votre liste principale, mais peuvent être consultées si l'autre personne envoie un nouveau message.

**Marquer comme lu :**
Les messages sont automatiquement marqués comme lus lorsque vous les consultez. Vous pouvez aussi marquer manuellement des conversations entières comme lues :
1. Ouvrez votre onglet Messages directs
2. Sélectionnez une conversation
3. Choisissez "Marquer comme lu"

Cela efface le compteur de non lus pour cette conversation.

### Bonnes pratiques

**Quand utiliser la Messagerie directe :**
- Poser des questions de suivi en privé
- Remercier quelqu'un pour un conseil utile
- Discuter de sujets hors-sujet sans encombrer les commentaires publics
- Se coordonner avec d'autres membres de la communauté
- Fournir des retours ou des suggestions en privé

**Étiquette des messages directs :**
- Soyez respectueux et professionnel
- Ne spammez pas les utilisateurs avec des messages non sollicités
- Respectez le fait que quelqu'un puisse ne pas répondre ou désactiver les DM
- Gardez les conversations pertinentes et constructives
- Ne partagez pas les DM d'autres personnes sans leur permission

**Conseils de sécurité :**
- Ne partagez pas d'informations personnelles (téléphone, adresse, etc.) à moins de faire confiance à la personne
- Bloquez et signalez les utilisateurs qui harcèlent ou abusent via DM
- Soyez prudent avec les liens ou demandes suspectes
- Utilisez la fonction de blocage si quelqu'un vous met mal à l'aise

### Limitations et remarques

**À qui pouvez-vous envoyer un message :**
- Tout utilisateur FastComments qui n'a pas désactivé les DM
- Les utilisateurs qui ne vous ont pas bloqué
- Les utilisateurs de toutes les communautés FastComments

**Contenu des messages :**
- Les messages texte sont pris en charge
- Les messages suivent les mêmes politiques de contenu que les commentaires
- Le contenu inapproprié peut être signalé

**Portée des conversations :**
- Les conversations DM sont privées entre deux personnes
- Pas de messagerie de groupe (actuellement uniquement en tête-à-tête)
- L'historique des conversations est conservé indéfiniment

### Dépannage

**Impossible d'envoyer un message ?**
L'autre utilisateur peut avoir :
- Désactivé les messages directs dans ses paramètres de confidentialité
- Vous avoir bloqué
- Supprimé son compte

**Vous ne recevez pas de notifications ?**
Vérifiez vos paramètres de notification dans [Paramètres du compte](https://fastcomments.com/auth/my-account/edit-notifications) pour vous assurer que les notifications DM sont activées.

**Les messages ne s'envoient pas ?**
- Vérifiez votre connexion Internet
- Actualisez la page et réessayez
- Assurez-vous de ne pas avoir été bloqué
- Contactez le support si le problème persiste