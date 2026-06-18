Cette bibliothèque est une implémentation complète react-native de [FastComments](https://fastcomments.com).

Il prend en charge les commentaires en direct, le chat, les fils de discussion, les émoticônes, les notifications, le SSO, les skins, et une personnalisation complète en passant un objet de feuille de style. Tous les assets
peuvent aussi être personnalisés, et il prend en charge l'activation de différents assets selon le mode sombre.

L'avantage de cette bibliothèque est qu'elle est plus flexible que le wrapper `fastcomments-react-native`. Les commentaires sont rendus avec des composants natifs plutôt qu'à l'intérieur d'une webview.

Tout fonctionne sur le backend de FastComments, donc vous n'avez qu'à intégrer l'interface utilisateur :

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Voir [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) pour plus d'exemples.

Ajoutez un chat en direct à votre application React Native existante, ou même créez un réseau social!