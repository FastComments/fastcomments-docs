Live threaded commenting with avatars, nested replies, votes, and the built-in rich-text composer, plus a dark theme and a live-chat preset (shown here rendered via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Comentarios en vivo</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-light.png" width="260" alt="Comentarios en vivo, tema claro"/></td>
    <td align="center"><b>Tema oscuro</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-dark.png" width="260" alt="Comentarios en vivo, tema oscuro"/></td>
    <td align="center"><b>Chat en vivo</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-chat.png" width="260" alt="Preset de chat en vivo"/></td>
  </tr>
</table>

### Editor de texto enriquecido

Esta biblioteca utiliza [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched-html) para la edición de texto enriquecido, lo que proporciona una potente experiencia de edición WYSIWYG. El mismo editor impulsa iOS, Android y la web (a través de `react-native-web`), por lo que el compositor se comporta de manera consistente en todas las plataformas con una única implementación.

`react-native-enriched` requiere la Nueva Arquitectura de React Native (Fabric) en nativo (el predeterminado desde RN 0.76, opcional en RN 0.72-0.75), y un empaquetador que resuelva las condiciones de `exports` del paquete. Este SDK se desarrolla y prueba con RN 0.81 / React 19. El mismo editor también se ejecuta en la web mediante `react-native-web`; la compilación web del editor enriquecido aún se marca como experimental en upstream.

### Widgets

El SDK incluye tres widgets, que replican el SDK de Android de FastComments:

- `FastCommentsLiveCommenting` - comentarios en hilos con votos, respuestas, paginación, menciones, notificaciones y actualizaciones en tiempo real.
- `FastCommentsLiveChat` - un preset de chat sobre el mismo motor: mensajes cronológicos con los nuevos al final, el compositor debajo de la lista, una barra de encabezado en vivo (punto de conexión + recuento de usuarios), historial infinito cargado al desplazarse hacia arriba, desplazamiento automático a los nuevos mensajes, sin votos ni hilos de respuesta. Cada preset puede sobrescribirse mediante `config`.
- `FastCommentsFeed` - un feed social con compositor de publicaciones, medios, reacciones, seguimientos y banners en vivo de nuevas publicaciones.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Tematización

El aspecto predeterminado se genera a partir de un conjunto de tokens de diseño semánticos (`FastCommentsTheme`): colores, espaciado, radio, tamaños de fuente, pesos de fuente y tamaños de avatar. Pase anulaciones parciales de tokens (tipado `FastCommentsThemeOverrides`) a través de la prop `theme` en cualquier widget y todo el árbol de estilos se restiliza de manera consistente:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

El modo oscuro está a un conjunto de tokens de distancia:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

La prop `styles` aún acepta un árbol `IFastCommentsStyles` sin procesar para un control quirúrgico. Cuando se proporcionan tanto `theme` como `styles`, los estilos explícitos prevalecen sobre el árbol tematizado; cuando solo se proporciona `styles`, reemplaza completamente los valores predeterminados (el comportamiento original, por lo que las integraciones y skins existentes no se ven afectadas). `setupDarkModeSkin` está en desuso en favor de la prop `theme`.

### Opciones de configuración

Esta biblioteca pretende soportar todas las opciones de configuración definidas en [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), al igual que la implementación web.

Sobre esas, React Native agrega algunas opciones específicas del SDK mediante `FastCommentsRNConfig`:

- `hideTopBar` - ocultar la barra superior del usuario conectado / campana de notificaciones mostrada encima del compositor.
- `usePressToEdit` - presionar y mantener un comentario para abrir su menú.
- `disableDownVoting` - ocultar los botones de voto negativo.
- `renderCommentInline` - renderizar la información del comentarista dentro del mismo bloque HTML que el contenido del comentario.
- `renderLikesToRight` - mover el área de voto/me gusta a la derecha del comentario en lugar de debajo.
- `renderDateBelowComment` - renderizar la fecha debajo del comentario.
- `showLiveStatus` - mostrar la barra de encabezado estilo chat "Live" + recuento de usuarios encima de los comentarios.
- `useInlineSubmitButton` - renderizar el botón de envío como un ícono dentro del compositor.
- `countAboveToggle` - con `useShowCommentsToggle`, cuántos comentarios se renderizan encima del interruptor "Mostrar comentarios".
- `preserveFeedScrollPosition` - `FastCommentsFeed` recuerda su desplazamiento de scroll entre desmontado y remontado (true por defecto).

### Conceptos de FastComments

Los conceptos principales a tener en cuenta para comenzar son `tenantId` y `urlId`. `tenantId` es el identificador de su cuenta en FastComments.com. `urlId` es donde se vincularán los hilos de comentarios. Esto podría ser la URL de una página, o un id de producto, un id de artículo, etc.

### Localización

Todo el texto visible para el usuario en estos widgets (etiquetas de botones, marcadores de posición, estados vacíos, fechas relativas como "hace 5 minutos", mensajes de error, etc.) es **controlado por el servidor**. Los componentes no codifican cadenas en inglés; renderizan las traducciones que FastComments sirve para la localidad solicitada.

Para solicitar una localidad, establezca `locale` en su configuración:

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, etc.
};
```

Cuando no se establece `locale`, FastComments sirve el idioma predeterminado del inquilino.

**Editar el texto:** las traducciones se gestionan en su panel de FastComments, no en este SDK. Para cambiar la redacción, sobrescriba el texto predeterminado, o añada un idioma, edite las traducciones de su cuenta en el panel – el cambio se aplica automáticamente a los widgets sin requerir una nueva versión de la aplicación. El SDK no incluye retrocompatibilidad en inglés, por lo que cualquier clave que deje en blanco en el panel se renderiza vacía; mantenga las claves pobladas para cada localidad que soporte.

### Notificaciones de usuario

FastComments soporta notificaciones para [muchos escenarios](https://docs.fastcomments.com/guide-notifications.html). Las notificaciones son configurables, pueden desactivarse globalmente o a nivel de notificación/comentario, y soportan suscripciones a nivel de página para que los usuarios puedan suscribirse a hilos de una página o artículo específico.

Por ejemplo, es posible usar Secure SSO para autenticar al usuario y luego sondear periódicamente las notificaciones no leídas y enviarlas al usuario.

Vea [el ejemplo AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) para saber cómo obtener y traducir notificaciones de usuario no leídas.

### Navegador de GIF

Por defecto, no se habilita la selección de imágenes o GIF. Consulte [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) para saber cómo admitir cargas de imágenes y GIF. Existe un Navegador de GIF que anonimiza búsquedas e imágenes provistas en esta biblioteca, simplemente tiene que usarlo.

### Rendimiento

Por favor, abra un ticket con un ejemplo para reproducir, incluyendo el dispositivo usado, si identifica algún problema de rendimiento. El rendimiento es un ciudadano de primera clase de todas las bibliotecas de FastComments.