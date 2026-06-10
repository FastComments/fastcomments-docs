#### Tema: Erebus
![Tema: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### Tema: Predeterminado
![Tema: Predeterminado](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### ¡Editor WYSIWYG nativo con soporte de imágenes!
![Editor WYSIWYG nativo con soporte de imágenes](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### Editor de texto enriquecido

Esta biblioteca usa [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) para la edición de texto enriquecido, lo que proporciona una potente experiencia de edición WYSIWYG. El mismo editor alimenta iOS, Android y la web (vía `react-native-web`), por lo que el compositor se comporta de forma coherente en todas las plataformas con una única implementación.

`react-native-enriched` requiere la React Native New Architecture (Fabric) en nativo, y un empaquetador que resuelva las condiciones de `exports` del paquete (Metro con package exports / RN 0.72+). El soporte para web es actualmente experimental.

### Opciones de configuración

Esta biblioteca pretende soportar todas las opciones de configuración definidas en [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), tal como la implementación web.

### Conceptos de FastComments

Los conceptos principales que debe conocer para empezar son `tenantId` y `urlId`. `tenantId` es el identificador de su cuenta en FastComments.com. `urlId` es a lo que se vincularán los hilos de comentarios. Esto podría ser una URL de página, o un id de producto, un id de artículo, etc.

### Notificaciones de usuario

FastComments admite notificaciones para [muchos escenarios](https://docs.fastcomments.com/guide-notifications.html). Las notificaciones son configurables, se puede optar por no recibirlas de forma global o a nivel de notificación/comentario, y admiten suscripciones a nivel de página para que los usuarios puedan suscribirse a los hilos de una página o artículo específico.

Por ejemplo, es posible usar Secure SSO para autenticar al usuario y luego verificar periódicamente si hay notificaciones no leídas y enviárselas al usuario.

Vea [el ejemplo AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) para ver cómo obtener y traducir las notificaciones de usuario no leídas.

### Explorador de GIFs

Por defecto, no está habilitada la selección de imágenes o gifs. Vea [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) para ver cómo admitir cargas de imágenes y gifs. En esta biblioteca hay un Explorador de GIFs que anonimiza las búsquedas y las imágenes proporcionadas; simplemente tiene que usarlo.

### Rendimiento

Por favor, abra un ticket con un ejemplo reproducible, incluyendo el dispositivo utilizado, si identifica algún problema de rendimiento. El rendimiento es una prioridad de primer orden en todas las bibliotecas de FastComments.