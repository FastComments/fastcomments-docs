#### Apariencia: Erebus
![Apariencia: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### Apariencia: Predeterminada
![Apariencia: Predeterminada](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### ¡Editor WYSIWYG nativo con soporte de imágenes!
![Editor WYSIWYG nativo con soporte de imágenes](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### Editor de Texto Enriquecido

Esta biblioteca utiliza el editor 10tap para la funcionalidad de edición de texto enriquecido, que ofrece una potente experiencia de edición WYSIWYG.

### Opciones de Configuración

Esta biblioteca tiene como objetivo soportar todas las opciones de configuración definidas en [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), al igual que la implementación web.

### Conceptos de FastComments

Los conceptos principales para comenzar son `tenantId` y `urlId`. `tenantId` es el identificador de tu cuenta en FastComments.com. `urlId` es el elemento al que estarán ligados los hilos de comentarios. Esto podría ser la URL de una página, o un ID de producto, un ID de artículo, etc.

### Notificaciones de Usuario

FastComments soporta notificaciones para [muchos escenarios](https://docs.fastcomments.com/guide-notifications.html). Las notificaciones son configurables, se puede optar por no recibirlas de forma global o a nivel de notificación/comentario, y soportan suscripciones a nivel de página para que los usuarios puedan suscribirse a los hilos de una página o artículo específico.

Por ejemplo, es posible usar Secure SSO para autenticar al usuario y luego sondear periódicamente las notificaciones no leídas y enviárselas al usuario.

Consulta [el ejemplo AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) para ver cómo obtener y traducir las notificaciones no leídas del usuario.

### Explorador de GIFs

Por defecto, no hay selección de imágenes o GIF habilitada. Consulta [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) para ver cómo soportar cargas de imágenes y GIFs. Existe un Explorador de GIFs en esta biblioteca que anonimiza las búsquedas y las imágenes proporcionadas; simplemente tienes que utilizarlo.

### Rendimiento

Por favor, abre un ticket con un ejemplo para reproducir el problema, incluyendo el dispositivo usado, si identificas algún problema de rendimiento. El rendimiento es una prioridad en todas las bibliotecas de FastComments.