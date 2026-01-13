Cette bibliothèque est une implémentation complète pour react-native de [FastComments](https://fastcomments.com).

Elle prend en charge les commentaires en direct, le chat, les threads, les émoticônes, les notifications, le SSO, les skins, et une personnalisation complète en passant un objet stylesheet. Tous les assets
peuvent également être personnalisés, et elle prend en charge le basculement entre différents assets en fonction du mode sombre.

Le principal avantage de cette bibliothèque est qu'elle est plus flexible et ne nécessite pas de webview, contrairement au wrapper `fastcomments-react-native`.

Tout fonctionne sur le backend FastComments, donc vous n'avez qu'à intégrer l'interface utilisateur :

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Voir [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) pour plus d'exemples.

Ajoutez le chat en direct à votre application React Native existante, ou même créez un réseau social !