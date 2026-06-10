Esta biblioteca es una implementación completa de react-native de [FastComments](https://fastcomments.com).

Admite comentarios en vivo, chat, hilos, emoticonos, notificaciones, SSO, temas, y personalización completa pasando un objeto de hoja de estilos. Todos los recursos
también se pueden personalizar, y admite alternar diferentes recursos según el modo oscuro.

La ventaja de esta biblioteca es que es más flexible que el wrapper `fastcomments-react-native`. Los comentarios se renderizan con componentes nativos en lugar de dentro de un webview.

Todo funciona en el backend de FastComments, por lo que solo tienes que incorporar la interfaz de usuario:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Consulta [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) para más ejemplos.

Añade chat en vivo a tu aplicación React Native existente, ¡o incluso crea una red social!