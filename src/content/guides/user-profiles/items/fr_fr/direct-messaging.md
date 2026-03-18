La messagerie directe (DM) permet aux utilisateurs de FastComments d'avoir des conversations privées, en tête-à-tête, distinctes des commentaires publics et des interactions de profil.

### Qu'est-ce que la messagerie directe?

La messagerie directe offre:
- **Conversations privées** entre deux utilisateurs
- **Messagerie en temps réel** avec notifications
- **Historique des conversations** qui persiste entre les sessions
- **Indicateurs de statut en ligne** pour voir quand quelqu'un est disponible
- **Suivi des messages non lus** pour rester informé des nouveaux messages

### Démarrer un message direct

Il existe plusieurs façons d'initier une conversation DM:

**Depuis le profil d'un utilisateur:**
1. Accédez à la page de profil de l'utilisateur
2. Cliquez sur l'onglet "Messagerie directe" ou le bouton "Envoyer un message"
3. Tapez votre message et envoyez-le

**Depuis un commentaire:**
Certaines implémentations permettent de cliquer sur le nom ou l'avatar d'un utilisateur pour accéder rapidement à son profil et aux options de messagerie.

**Lien direct:**
Si vous disposez d'un identifiant de conversation ou d'un lien de redirection DM, vous pouvez accéder directement aux conversations.

### Accéder à vos messages

Pour consulter toutes vos conversations en messages directs:

1. Accédez à votre page de profil
2. Cliquez sur l'onglet "Messagerie directe"
3. Consultez la liste de toutes vos conversations

Chaque conversation affiche:
- L'avatar et le nom de l'autre participant
- Leur statut en ligne/hors ligne (indicateur vert lorsqu'ils sont en ligne)
- Un aperçu du dernier message
- Nombre de messages non lus (le cas échéant)
- Horodatage de la dernière activité

### Vue de la conversation

Lorsque vous ouvrez une conversation, vous verrez:

- **Historique complet des messages** - Tous les messages entre vous et l'autre utilisateur
- **Mises à jour en temps réel** - Les nouveaux messages apparaissent instantanément via WebSocket
- **Statut en ligne** - Voir si l'autre personne est actuellement en ligne
- **Horodatage des messages** - Quand chaque message a été envoyé
- **Rédaction de message** - Zone de texte pour saisir et envoyer de nouveaux messages

### Notifications de messages

Vous recevrez des notifications pour les nouveaux messages directs:

- **Notifications dans l'application** - Compteur de badge sur votre profil
- **Notifications par e-mail** - Selon vos paramètres de notification
- **Alertes en temps réel** - Notifications instantanées lorsque vous êtes connecté

Gérez vos préférences de notification dans vos [Paramètres du compte](https://fastcomments.com/auth/my-account/edit-notifications).

### Confidentialité et blocage

**Désactiver les messages directs:**
Si vous ne souhaitez pas recevoir de DM, vous pouvez les désactiver complètement:
1. Accédez aux paramètres de confidentialité de votre profil
2. Activez "Désactiver les messages directs"
3. L'option DM sera masquée de votre profil

Pour plus de détails, voir [Paramètres de confidentialité](/guide-user-profiles.html#privacy-settings).

**Bloquer des utilisateurs:**
Si quelqu'un vous harcèle via DM:
1. Visitez leur profil
2. Bloquez l'utilisateur
3. Ils ne pourront plus vous envoyer de messages

Les utilisateurs bloqués ne peuvent pas:
- Vous envoyer des messages directs
- Voir votre activité
- Commenter votre profil
- Interagir avec vous n'importe où sur FastComments

### Gérer les conversations

**Masquer des conversations:**
Vous pouvez masquer les conversations que vous ne souhaitez plus voir dans votre liste de conversations:
1. Ouvrez votre onglet Messagerie directe
2. Trouvez la conversation à masquer
3. Sélectionnez l'option masquer/archiver

Les conversations masquées n'apparaîtront pas dans votre liste principale mais peuvent être consultées si l'autre personne envoie un nouveau message.

**Marquer comme lu:**
Les messages sont automatiquement marqués comme lus lorsque vous les consultez. Vous pouvez également marquer manuellement des conversations entières comme lues:
1. Ouvrez votre onglet Messagerie directe
2. Sélectionnez une conversation
3. Choisissez "Marquer comme lu"

Cela efface le compteur de non lus pour cette conversation.

### Bonnes pratiques

**Quand utiliser les messages directs:**
- Poser des questions de suivi en privé
- Remercier quelqu'un pour un conseil utile
- Discuter de sujets hors-sujet sans encombrer les commentaires publics
- Se coordonner avec d'autres membres de la communauté
- Fournir des retours ou suggestions privés

**Étiquette des DM:**
- Soyez respectueux et professionnel
- N'envoyez pas de messages non sollicités ou du spam aux utilisateurs
- Respectez le fait que quelqu'un ne réponde pas ou qu'il désactive les DM
- Gardez les conversations pertinentes et constructives
- Ne partagez pas les DM d'autres personnes sans autorisation

**Conseils de sécurité:**
- Ne partagez pas d'informations personnelles (téléphone, adresse, etc.) à moins de faire confiance à la personne
- Bloquez et signalez les utilisateurs qui harcèlent ou abusent des DM
- Soyez prudent avec les liens ou demandes suspects
- Utilisez la fonction de blocage si quelqu'un vous met mal à l'aise

### Limitations et remarques

**À qui pouvez-vous envoyer des messages:**
- Tout utilisateur FastComments qui n'a pas désactivé les DM
- Les utilisateurs qui ne vous ont pas bloqué
- Les utilisateurs de toutes les communautés FastComments

**Contenu des messages:**
- Les messages texte sont pris en charge
- Les messages suivent les mêmes règles de contenu que les commentaires
- Le contenu inapproprié peut être signalé

**Portée des conversations:**
- Les conversations DM sont privées et se déroulent entre deux personnes
- Pas de discussion de groupe (actuellement uniquement en tête-à-tête)
- L'historique des conversations est conservé indéfiniment

### Dépannage

**Impossible d'envoyer un message?**
L'autre utilisateur peut avoir:
- Désactivé les messages directs dans ses paramètres de confidentialité
- Vous bloqué
- Supprimé son compte

**Vous ne recevez pas de notifications?**
Vérifiez vos paramètres de notification dans [Paramètres du compte](https://fastcomments.com/auth/my-account/edit-notifications) pour vous assurer que les notifications DM sont activées.

**Les messages ne s'envoient pas?**
- Vérifiez votre connexion Internet
- Actualisez la page et réessayez
- Assurez-vous de ne pas avoir été bloqué
- Contactez le support si le problème persiste