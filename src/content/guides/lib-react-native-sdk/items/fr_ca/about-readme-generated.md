Cette bibliothèque est une implémentation complète de FastComments pour react-native.

Elle prend en charge les commentaires en direct, le chat, les fils de discussion, les émoticônes, les notifications, le SSO, les skins, et une personnalisation complète en passant un objet de feuille de style. Tous les assets
peuvent aussi être personnalisés, et elle prend en charge la commutation d'assets différents selon le mode sombre.

L'avantage de cette bibliothèque est qu'elle est plus flexible que le `fastcomments-react-native` wrapper. Les commentaires sont rendus avec des composants natifs plutôt qu'à l'intérieur d'une webview. Remarque : `react-native-webview` est toujours requis comme dépendance transitive de l'éditeur de texte enrichi (`@10play/tentap-editor`).

Tout fonctionne sur le backend FastComments, donc vous n'avez qu'à intégrer l'interface utilisateur :

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Voir [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) pour plus d'exemples.

Ajoutez un chat en direct à votre application React Native existante, ou même construisez un réseau social !