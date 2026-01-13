La messagerie directe (MD) permet aux utilisateurs de FastComments d'avoir des conversations privées en tête-à-tête, séparées des commentaires publics et des interactions de profil.

### Qu'est-ce que la messagerie directe ?

La messagerie directe offre :
- **Des conversations privées** entre deux utilisateurs
- **Des messages en temps réel** avec notifications
- **Un historique des conversations** qui persiste entre les sessions
- **Des indicateurs de statut en ligne** pour voir quand quelqu'un est disponible
- **Le suivi des messages non lus** pour rester à jour sur les nouveaux messages

### Démarrer une messagerie directe

Il existe plusieurs façons d'initier une conversation MD :

**Depuis un profil utilisateur :**
1. Visitez la page de profil de l'utilisateur
2. Cliquez sur l'onglet « Messagerie directe » ou le bouton « Envoyer un message »
3. Tapez votre message et envoyez

**Depuis un commentaire :**
Certaines implémentations permettent de cliquer sur le nom ou l'avatar d'un utilisateur pour accéder rapidement à son profil et aux options de messagerie.

**Lien direct :**
Si vous avez un identifiant de conversation ou un lien de redirection MD, vous pouvez accéder directement aux conversations.

### Accéder à vos messages

Pour voir toutes vos conversations de messagerie directe :

1. Visitez votre page de profil
2. Cliquez sur l'onglet « Messagerie directe »
3. Voyez la liste de toutes vos conversations

Chaque conversation affiche :
- L'avatar et le nom de l'autre participant
- Leur statut en ligne/hors ligne (indicateur vert lorsque en ligne)
- Un aperçu du dernier message
- Le nombre de messages non lus (le cas échéant)
- L'horodatage de la dernière activité

### Vue de la conversation

Lorsque vous ouvrez une conversation, vous verrez :

- **L'historique complet des messages** - Tous les messages entre vous et l'autre utilisateur
- **Mises à jour en temps réel** - Les nouveaux messages apparaissent instantanément via WebSocket
- **Statut en ligne** - Voyez si l'autre personne est actuellement en ligne
- **Horodatages des messages** - Quand chaque message a été envoyé
- **Composition de message** - Zone de texte pour taper et envoyer de nouveaux messages

### Notifications de messages

Vous recevrez des notifications pour les nouveaux messages directs :

- **Notifications intégrées** - Compteur de badge sur votre profil
- **Notifications par courriel** - Selon vos paramètres de notification
- **Alertes en temps réel** - Notifications instantanées lorsque vous êtes connecté

Gérez les préférences de notification dans vos [Paramètres du compte](https://fastcomments.com/auth/my-account/notification-settings).

### Confidentialité et blocage

**Désactiver la messagerie directe :**
Si vous ne souhaitez pas recevoir de MD, vous pouvez les désactiver complètement :
1. Allez dans les paramètres de confidentialité de votre profil
2. Activez « Désactiver la messagerie directe »
3. L'option MD sera masquée de votre profil

Pour plus de détails, voir [Paramètres de confidentialité](/guides/user-profiles/privacy-settings).

**Bloquer des utilisateurs :**
Si quelqu'un vous harcèle via MD :
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
Vous pouvez masquer les conversations que vous ne souhaitez plus voir dans votre liste de conversations :
1. Ouvrez votre onglet Messagerie directe
2. Trouvez la conversation à masquer
3. Sélectionnez l'option masquer/archiver

Les conversations masquées n'apparaîtront pas dans votre liste principale mais pourront être consultées si l'autre personne envoie un nouveau message.

**Marquer comme lu :**
Les messages sont automatiquement marqués comme lus lorsque vous les consultez. Vous pouvez aussi marquer manuellement des conversations entières comme lues :
1. Ouvrez votre onglet Messagerie directe
2. Sélectionnez une conversation
3. Choisissez « Marquer comme lu »

Cela efface le compteur de messages non lus pour cette conversation.

### Bonnes pratiques

**Quand utiliser la messagerie directe :**
- Poser des questions de suivi en privé
- Remercier quelqu'un pour des conseils utiles
- Discuter de sujets hors-sujet sans encombrer les commentaires publics
- Se coordonner avec d'autres membres de la communauté
- Fournir des commentaires ou des suggestions privées

**Netiquette pour la messagerie directe :**
- Soyez respectueux et professionnel
- N'envoyez pas de messages non sollicités en masse
- Respectez le fait que quelqu'un puisse ne pas répondre ou désactiver la MD
- Gardez les conversations pertinentes et constructives
- Ne partagez pas les messages privés d'autres personnes sans permission

**Conseils de sécurité :**
- Ne partagez pas d'informations personnelles (téléphone, adresse, etc.) sauf si vous faites confiance à la personne
- Bloquez et signalez les utilisateurs qui harcèlent ou abusent via MD
- Méfiez-vous des liens ou demandes suspects
- Utilisez la fonction de blocage si quelqu'un vous met mal à l'aise

### Limites et remarques

**Qui pouvez-vous contacter :**
- Tout utilisateur de FastComments qui n'a pas désactivé la MD
- Les utilisateurs qui ne vous ont pas bloqué
- Les utilisateurs de toutes les communautés FastComments

**Contenu des messages :**
- Les messages texte sont pris en charge
- Les messages suivent les mêmes politiques de contenu que les commentaires
- Le contenu inapproprié peut être signalé

**Portée de la conversation :**
- Les conversations MD sont privées entre deux personnes
- Pas de messages de groupe (actuellement uniquement en tête-à-tête)
- L'historique des conversations est conservé indéfiniment

### Dépannage

**Impossible d'envoyer un message ?**
L'autre utilisateur peut avoir :
- Désactivé la messagerie directe dans ses paramètres de confidentialité
- Vous avoir bloqué
- Supprimé son compte

**Vous ne recevez pas de notifications ?**
Vérifiez vos paramètres de notification dans les [Paramètres du compte](https://fastcomments.com/auth/my-account/notification-settings) pour vous assurer que les notifications MD sont activées.

**Les messages ne s'envoient pas ?**
- Vérifiez votre connexion Internet
- Actualisez la page et réessayez
- Assurez-vous que vous n'avez pas été bloqué
- Contactez le support si le problème persiste