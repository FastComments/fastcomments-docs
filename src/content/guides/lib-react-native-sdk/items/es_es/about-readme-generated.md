Esta biblioteca es una implementación completa para react-native de [FastComments](https://fastcomments.com).

Admite comentarios en vivo, chat, hilos, emoticonos, notificaciones, SSO, skins y personalización completa pasando un objeto de hoja de estilos. Todos los recursos
también se pueden personalizar, y admite alternar diferentes recursos según el modo oscuro.

La ventaja de esta biblioteca es que es más flexible que el envoltorio `fastcomments-react-native`. Los comentarios se renderizan con componentes nativos en lugar de dentro de un webview. Nota: `react-native-webview` sigue siendo necesario como dependencia transitiva del editor de texto enriquecido (`@10play/tentap-editor`).

Todo se ejecuta en el backend de FastComments, por lo que solo tiene que incorporar la interfaz de usuario:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Vea [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) para más ejemplos.

Añada chat en vivo a su aplicación React Native existente, ¡o incluso construya una red social!