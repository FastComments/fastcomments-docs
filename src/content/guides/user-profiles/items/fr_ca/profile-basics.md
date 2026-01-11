Les profils d'utilisateur dans FastComments offrent un espace dédié à chaque utilisateur pour mettre en valeur son identité, son activité et ses contributions au sein de votre communauté.

### Qu'est-ce qu'un profil d'utilisateur ?

Un profil d'utilisateur est une page personnalisée pour chaque utilisateur FastComments qui affiche :

- **Profile Header** - Une image d'en-tête personnalisable qui personnalise le profil
- **Avatar** - La photo de profil de l'utilisateur avec un indicateur de statut en ligne/hors ligne
- **Display Information** - Nom d'utilisateur, nom affiché et drapeau du pays optionnel
- **Bio** - Une description personnelle ou une introduction
- **Social Links** - Liens vers les profils de réseaux sociaux de l'utilisateur et son site web
- **Badges** - Récompenses et reconnaissances obtenues
- **Statistics** - Karma de l'utilisateur et nombre total de commentaires
- **Communities** - Sur quels sites/domaines l'utilisateur est actif

### Accéder aux profils d'utilisateur

Il existe plusieurs façons d'accéder au profil d'un utilisateur :

1. **Click on an avatar** - Dans le widget de commentaires, cliquez sur l'avatar de n'importe quel utilisateur pour voir son profil
2. **Click on a username** - Les noms d'utilisateur dans les commentaires sont des liens cliquables vers les profils
3. **Direct URL** - Visitez `https://fastcomments.com/auth/user-profile/[userId]`

### Vues du profil

Lorsque vous consultez un profil, vous verrez différents onglets selon que vous visualisez votre propre profil ou celui de quelqu'un d'autre :

#### Votre propre profil
- **Notifications** - Vos notifications et mentions
- **Recent Activity** - Votre historique de commentaires sur toutes les communautés
- **Profile Comments** - Commentaires que d'autres ont laissés sur votre page de profil
- **Direct Messages** - Conversations privées avec d'autres utilisateurs

#### Profils des autres utilisateurs
- **Recent Activity** - Leur historique public de commentaires (s'il n'est pas défini comme privé)
- **Profile Comments** - Commentaires sur leur profil (s'ils ne sont pas définis comme privés)
- **Direct Messages** - Commencer ou poursuivre une conversation privée (s'ils autorisent les messages privés)

### Statut en ligne

Les profils d'utilisateur affichent le statut en ligne en temps réel :
- **Green indicator** - L'utilisateur est actuellement en ligne
- **No indicator** - L'utilisateur est hors ligne

Cela vous aide à savoir quand quelqu'un utilise activement la plateforme, ce qui est particulièrement utile pour les messages privés.

### Types d'utilisateurs

FastComments prend en charge deux types d'utilisateurs avec des profils :

1. **Regular Users** - Utilisateurs qui se sont inscrits directement auprès de FastComments
2. **SSO Users** - Utilisateurs qui s'authentifient via l'intégration Single Sign-On de votre site

Les deux types d'utilisateurs ont accès au système de profils complet, bien que les utilisateurs SSO puissent avoir certaines restrictions pour modifier certains champs (comme les avatars) selon la configuration de votre SSO.