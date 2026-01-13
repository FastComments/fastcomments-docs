Les profils d'utilisateur dans FastComments offrent un espace dédié pour chaque utilisateur afin de mettre en valeur son identité, son activité et ses contributions au sein de votre communauté.

### Qu'est-ce qu'un profil utilisateur ?

Un profil utilisateur est une page personnalisée pour chaque utilisateur FastComments qui affiche :

- **En-tête du profil** - Une image d'arrière-plan personnalisable qui personnalise le profil
- **Avatar** - La photo de profil de l'utilisateur avec un indicateur de statut en ligne/hors ligne
- **Informations d'affichage** - Nom d'utilisateur, nom affiché et drapeau du pays optionnel
- **Bio** - Une description personnelle ou une présentation
- **Liens sociaux** - Connexions aux profils de réseaux sociaux et au site web de l'utilisateur
- **Badges** - Récompenses et reconnaissances obtenues
- **Statistiques** - Karma de l'utilisateur et nombre total de commentaires
- **Communautés** - Sur quels sites/domaines l'utilisateur est actif

### Accéder aux profils utilisateur

Il existe plusieurs façons d'accéder au profil d'un utilisateur :

1. **Cliquez sur un avatar** - Dans le widget de commentaires, cliquez sur l'avatar de n'importe quel utilisateur pour voir son profil
2. **Cliquez sur un nom d'utilisateur** - Les noms d'utilisateur dans les commentaires sont des liens cliquables vers les profils
3. **URL directe** - Visitez `https://fastcomments.com/auth/user-profile/[userId]`

### Vues du profil

Lorsque vous consultez un profil, vous verrez différents onglets selon que vous consultez votre propre profil ou celui d'un autre utilisateur :

#### Votre propre profil
- **Notifications** - Vos notifications et mentions
- **Activité récente** - Votre historique de commentaires sur toutes les communautés
- **Commentaires sur le profil** - Commentaires que d'autres ont laissés sur votre page de profil
- **Messages directs** - Conversations privées avec d'autres utilisateurs

#### Profils des autres utilisateurs
- **Activité récente** - Leur historique public de commentaires (si le profil n'est pas défini comme privé)
- **Commentaires sur le profil** - Commentaires sur leur profil (si le profil n'est pas défini comme privé)
- **Messages directs** - Démarrer ou poursuivre une conversation privée (s'ils autorisent les messages privés)

### Statut en ligne

Les profils utilisateur affichent le statut en ligne en temps réel :
- **Indicateur vert** - L'utilisateur est actuellement en ligne
- **Pas d'indicateur** - L'utilisateur est hors ligne

Cela vous aide à savoir quand quelqu'un utilise activement la plateforme, ce qui est particulièrement utile pour les messages directs.

### Types d'utilisateurs

FastComments prend en charge deux types d'utilisateurs disposant de profils :

1. **Utilisateurs réguliers** - Utilisateurs qui se sont inscrits directement auprès de FastComments
2. **Utilisateurs SSO** - Utilisateurs qui s'authentifient via l'intégration Single Sign-On (SSO) de votre site

Les deux types d'utilisateurs ont accès au système de profil complet, bien que les utilisateurs SSO puissent avoir certaines restrictions pour modifier certains champs (comme les avatars) en fonction de la configuration de votre SSO.