Cette bibliothèque est une implémentation complète de [FastComments](https://fastcomments.com) pour react-native.

Elle prend en charge les commentaires en direct, le chat, les fils de discussion, les émoticônes, les notifications, le SSO, les thèmes, et une personnalisation complète en passant un objet de feuille de style. Toutes les ressources
peuvent également être personnalisées, et elle prend en charge l'activation de différentes ressources en fonction du mode sombre.

L'avantage de cette bibliothèque est qu'elle est plus flexible que le wrapper `fastcomments-react-native`. Les commentaires sont rendus avec des composants natifs plutôt qu'à l'intérieur d'une webview.

Tout fonctionne sur le backend FastComments, vous n'avez donc qu'à intégrer l'interface utilisateur :

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Voir [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) pour plus d'exemples.

Ajoutez un chat en direct à votre application React Native existante, ou créez même un réseau social !