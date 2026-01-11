Lorsque les profils d'utilisateur sont ouverts dans le contexte de votre site (via le widget de commentaires), toutes les feuilles de style CSS personnalisées que vous avez appliquées à votre widget FastComments sont automatiquement injectées dans la modale de profil.

### Comment ça fonctionne

Lorsqu'un utilisateur clique sur un lien de profil dans votre widget de commentaires, une modale de profil s'ouvre avec la classe `.fast-comments-profile`. Le CSS personnalisé de votre widget est automatiquement injecté dans la vue du profil. Si vous avez déjà stylisé votre widget de commentaires, ces styles s'appliqueront aux profils.

### Classes CSS

Les profils FastComments utilisent une architecture CSS basée sur des classes. Elle n'utilise pas de propriétés CSS personnalisées.

La page de profil principale utilise `.user-profile` comme conteneur racine. La section d'en-tête est `.profile-header` avec `.profile-header-background` pour l'image de fond. Le contenu du profil se trouve dans `.profile-content`.

L'avatar utilise `.profile-avatar` et `.profile-avatar-wrapper`. Le nom de l'utilisateur est `.profile-name` et le texte de la biographie est `.profile-bio`. Les statistiques se trouvent dans `.profile-stats` avec des statistiques individuelles utilisant `.stat`.

Les liens sociaux se trouvent dans `.profile-social-links` avec des liens individuels en tant que `.social-link`. Les badges utilisent `.profile-badges` et `.badge`. Les barres de progression des badges utilisent `.progress-outer` et `.progress-bar`.

Les onglets utilisent `.profile-tabs` pour le conteneur, `.tab` pour les onglets individuels, et `.tab.active` pour l'onglet sélectionné. Le contenu des onglets utilise `.tab-body` et `.tab-body.active`. Les comptes de notifications sur les onglets utilisent `.tab .count`.

Les notifications utilisent `.notification` et les conversations DM utilisent `.conversation`. L'état en ligne est `.activity-indicator` avec `.activity-indicator.online` pour l'état actif. Les compteurs de non lus utilisent `.unread-count`.

Le conteneur de la modale de profil est `.fast-comments-profile` avec `.fast-comments-profile-close` pour le bouton de fermeture.

### Mode sombre

Le mode sombre utilise le modificateur de classe `.dark` sur `.user-profile`.

```css
.user-profile.dark {
    background-color: #181a1b;
    color: #fff;
}
```

### Exemples

**En-tête :**
```css
.user-profile .profile-header-background {
    background: linear-gradient(to right, #667eea, #764ba2);
}
```

**Badges :**
```css
.user-profile .badge {
    background: #007bff;
    color: white;
    border-radius: 24px;
}
```

**Onglets :**
```css
.user-profile .tab.active {
    color: #007bff;
    border-bottom: 3px solid #007bff;
}
```

**Modale :**
```css
.fast-comments-profile {
    border-radius: 12px 0 0 12px;
}
```