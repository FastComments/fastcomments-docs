Live threaded commenting with avatars, nested replies, votes, and the built-in rich-text composer, plus a dark theme and a live-chat preset (shown here rendered via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Comentarios en vivo</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-light.png" width="260" alt="Comentarios en vivo, tema claro"/></td>
    <td align="center"><b>Tema oscuro</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-dark.png" width="260" alt="Comentarios en vivo, tema oscuro"/></td>
    <td align="center"><b>Chat en vivo</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-chat.png" width="260" alt="Preset de chat en vivo"/></td>
  </tr>
</table>

### Editor de texto enriquecido

Esta biblioteca usa [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) para la edición de texto enriquecido, lo que brinda una potente experiencia de edición WYSIWYG. El mismo editor impulsa iOS, Android y la web (a través de `react-native-web`), por lo que el compositor se comporta de manera consistente en todas las plataformas con una única implementación.

`react-native-enriched` requiere la Nueva Arquitectura de React Native (Fabric) en nativo (el predeterminado desde RN 0.76, opcional en RN 0.72-0.75), y un empaquetador que resuelva las condiciones de `exports` del paquete. Este SDK se desarrolla y prueba contra RN 0.81 / React 19. El mismo editor también se ejecuta en la web a través de `react-native-web`; la compilación web del editor enriquecido aún se marca como experimental en upstream.

### Widgets

- `FastCommentsLiveCommenting` - comentarios en hilos con votos, respuestas, paginación, menciones, notificaciones y actualizaciones en tiempo real.
- `FastCommentsLiveChat` - un preset de chat sobre el mismo motor: mensajes cronológicos con los nuevos al final, el compositor debajo de la lista, una barra de encabezado en vivo (punto de conexión + recuento de usuarios), historial infinito cargado al desplazarse hacia arriba, desplazamiento automático a los nuevos mensajes, sin votos ni hilos de respuesta. Cada preset puede sobrescribirse mediante `config`.
- `FastCommentsFeed` - un feed social con compositor de publicaciones, medios, reacciones, seguimientos y banners en vivo de nuevas publicaciones.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Tematización

El aspecto predeterminado se genera a partir de un conjunto de tokens de diseño semántico (`FastCommentsTheme`): colores, espaciado, radio, tamaños de fuente, pesos de fuente y tamaños de avatar. Pasa anulaciones parciales de tokens (tipado `FastCommentsThemeOverrides`) a través de la prop `theme` en cualquier widget y todo el árbol de estilos se vuelve a aplicar de manera consistente:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

El modo oscuro está a un conjunto de tokens de distancia:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

La prop `styles` aún acepta un árbol `IFastCommentsStyles` sin procesar para un control quirúrgico. Cuando se proporcionan tanto `theme` como `styles`, los estilos explícitos prevalecen sobre el árbol temático; cuando solo se proporciona `styles`, reemplaza completamente los valores predeterminados (el comportamiento original, por lo que las integraciones y skins existentes no se ven afectadas). `setupDarkModeSkin` está en desuso en favor de la prop `theme`.

### Opciones de configuración

Esta biblioteca pretende soportar todas las opciones de configuración definidas en [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), al igual que la implementación web.

Además de esas, React Native agrega algunas opciones específicas del SDK mediante `FastCommentsRNConfig`:

- `hideTopBar` - oculta la barra superior del usuario conectado / campana de notificaciones mostrada encima del compositor.
- `usePressToEdit` - presiona y mantén pulsado un comentario para abrir su menú.
- `disableDownVoting` - oculta los botones de voto negativo.
- `renderCommentInline` - renderiza la información del comentarista dentro del mismo bloque HTML que el contenido del comentario.
- `renderLikesToRight` - mueve el área de voto/me gusta a la derecha del comentario en lugar de debajo.
- `renderDateBelowComment` - muestra la fecha debajo del comentario.
- `showLiveStatus` - muestra la barra de encabezado estilo chat "Live" + recuento de usuarios encima de los comentarios.
- `useInlineSubmitButton` - renderiza el botón de envío como un ícono dentro del compositor.
- `countAboveToggle` - con `useShowCommentsToggle`, cuántos comentarios se renderizan encima del interruptor "Mostrar comentarios".
- `preserveFeedScrollPosition` - `FastCommentsFeed` recuerda su desplazamiento de scroll entre desmontajes y remontajes (verdadero por defecto).

### Conceptos de FastComments

Los conceptos principales que debes conocer para comenzar son `tenantId` y `urlId`. `tenantId` es el identificador de tu cuenta en FastComments.com. `urlId` es a lo que se vincularán los hilos de comentarios. Esto podría ser la URL de una página, o un ID de producto, un ID de artículo, etc.

### Localización

Todo el texto visible para el usuario en estos widgets (etiquetas de botones, marcadores de posición, estados vacíos, fechas relativas como "hace 5 minutos", mensajes de error, etc.) es **controlado por el servidor**. Los componentes no codifican cadenas en inglés; renderizan las traducciones que FastComments proporciona para la localidad solicitada.

Para solicitar una localidad, establece `locale` en tu configuración:

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, etc.
};
```

Cuando no se establece `locale`, FastComments sirve el idioma predeterminado del inquilino.

**Editando el texto:** las traducciones se gestionan en el panel de control de FastComments, no en este SDK. Para cambiar la redacción, sobrescribe el texto predeterminado, o agrega un idioma, edita las traducciones de tu cuenta en el panel; el cambio es detectado automáticamente por los widgets sin requerir una nueva versión de la aplicación. El SDK no incluye valores predeterminados en inglés, por lo que cualquier clave que dejes en blanco en el panel se mostrará vacía; mantén las claves pobladas para cada localidad que soportes.

### Notificaciones de usuario

FastComments soporta notificaciones para [muchos escenarios](https://docs.fastcomments.com/guide-notifications.html). Las notificaciones son configurables, pueden desactivarse globalmente o a nivel de notificación/comentario, y soportan suscripciones a nivel de página para que los usuarios puedan suscribirse a hilos de una página o artículo específico.

Por ejemplo, es posible usar Secure SSO para autenticar al usuario y luego sondear periódicamente las notificaciones no leídas y enviárselas al usuario.

Consulta [el ejemplo AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) para saber cómo obtener y traducir las notificaciones de usuario no leídas.

### Navegador de GIF

Por defecto, no se habilita la selección de imágenes o GIF. Consulta [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) para saber cómo admitir cargas de imágenes y GIF. Existe un Navegador de GIF que anonimiza las búsquedas e imágenes proporcionadas en esta biblioteca; simplemente tienes que usarlo.

### Rendimiento

Por favor, abre un ticket con un ejemplo para reproducir el problema, incluyendo el dispositivo usado, si identificas algún problema de rendimiento. El rendimiento es una prioridad de primera clase en todas las bibliotecas de FastComments.