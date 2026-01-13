Cette bibliothèque est une implémentation complète de react-native de [FastComments](https://fastcomments.com).

Elle prend en charge les commentaires en direct, le chat, les fils de discussion, les émoticônes, les notifications, le SSO, les thèmes (skins) et une personnalisation complète en passant un objet feuille de style. Toutes les ressources
peuvent également être personnalisées, et elle permet de basculer entre différentes ressources selon le mode sombre.

L'avantage de cette bibliothèque est qu'elle est plus flexible et qu'elle ne nécessite pas de webview, contrairement au wrapper `fastcomments-react-native`.

Tout s'exécute sur le backend FastComments, vous n'avez donc qu'à intégrer l'interface utilisateur :

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Voir [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) pour plus d'exemples.

Ajoutez un chat en direct à votre application React Native existante, ou construisez même un réseau social !