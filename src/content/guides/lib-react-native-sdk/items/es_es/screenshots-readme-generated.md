Comentarios en hilos en vivo con avatares, respuestas anidadas, votos y el compositor enriquecido integrado, además de un tema oscuro y un ajuste predefinido de chat en vivo (mostrado aquí renderizado mediante `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Comentarios en vivo</b><br/><img src="./demo-screenshots/light.png" width="260" alt="Comentarios en vivo, tema claro"/></td>
    <td align="center"><b>Tema oscuro</b><br/><img src="./demo-screenshots/dark.png" width="260" alt="Comentarios en vivo, tema oscuro"/></td>
    <td align="center"><b>Chat en vivo</b><br/><img src="./demo-screenshots/chat.png" width="260" alt="Ajuste predefinido de chat en vivo"/></td>
  </tr>
</table>

### Editor de Texto Enriquecido

Esta librería usa [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) para la edición de texto enriquecido, lo que proporciona una experiencia WYSIWYG potente. El mismo editor alimenta iOS, Android y la web (vía `react-native-web`), por lo que el compositor se comporta de forma consistente en todas las plataformas con una única implementación.

`react-native-enriched` requiere la React Native New Architecture (Fabric) en nativo (el valor predeterminado desde RN 0.76, opt-in en RN 0.72-0.75), y un empaquetador que resuelva las condiciones de `exports` del paquete. Este SDK se desarrolla y prueba contra RN 0.81 / React 19. El mismo editor también se ejecuta en la web a través de `react-native-web`; la build web del editor enriched aún se marca como experimental en el upstream.

### Widgets

El SDK incluye tres widgets, reflejando el SDK de FastComments para Android:

- `FastCommentsLiveCommenting` - comentarios en hilos con votos, respuestas, paginación, menciones, notificaciones y actualizaciones en vivo.
- `FastCommentsLiveChat` - un ajuste predefinido de chat sobre el mismo motor: mensajes cronológicos con los nuevos al final, el compositor debajo de la lista, una franja de encabezado en vivo (punto de conexión + conteo de usuarios), historial infinito cargado al desplazarse hacia arriba, auto-scroll a mensajes nuevos, sin votos ni estructura de respuestas. Cada preset puede ser sobrescrito vía `config`.
- `FastCommentsFeed` - un feed social con compositor de publicaciones, medios, reacciones, seguimientos y banners de nuevas publicaciones en vivo.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Temas

La apariencia predeterminada se genera a partir de un conjunto de tokens de diseño semánticos (`FastCommentsTheme`): colores, espaciados, radios, tamaños de fuente, pesos de fuente y tamaños de avatar. Pasa overrides parciales de tokens (tipados `FastCommentsThemeOverrides`) a través de la prop `theme` en cualquier widget y todo el árbol de estilos se restilará de manera consistente:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

El modo oscuro está a un conjunto de tokens de distancia:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

La prop `styles` todavía acepta un árbol crudo `IFastCommentsStyles` para control quirúrgico. Cuando `theme` y `styles` se proporcionan ambos, los estilos explícitos prevalecen sobre el árbol temático; cuando solo se proporciona `styles`, reemplaza por completo los valores predeterminados (el comportamiento original, por lo que las integraciones y skins existentes no se ven afectadas). `setupDarkModeSkin` está obsoleto a favor de la prop `theme`.

### Opciones de Configuración

Esta librería pretende soportar todas las opciones de configuración definidas en [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), igual que la implementación web.

Además de esas, React Native añade algunas opciones específicas del SDK vía `FastCommentsRNConfig`:

- `hideTopBar` - ocultar la franja que muestra el usuario conectado / campana de notificaciones encima del compositor.
- `usePressToEdit` - mantener presionado un comentario para abrir su menú.
- `disableDownVoting` - ocultar los botones de voto negativo.
- `renderCommentInline` - renderizar la información del comentarista dentro del mismo bloque HTML que el contenido del comentario.
- `renderLikesToRight` - mover el área de votos/likes a la derecha del comentario en lugar de debajo.
- `renderDateBelowComment` - renderizar la fecha debajo del comentario.
- `showLiveStatus` - mostrar la franja de encabezado estilo chat "Live" + conteo de usuarios encima de los comentarios.
- `useInlineSubmitButton` - renderizar el botón de envío como un icono dentro del compositor.
- `countAboveToggle` - con `useShowCommentsToggle`, cuántos comentarios se renderizan encima del toggle "Show Comments".
- `preserveFeedScrollPosition` - `FastCommentsFeed` recuerda su offset de scroll entre desmontajes/volver a montar (por defecto true).

### Conceptos de FastComments

Los conceptos principales a tener en cuenta para comenzar son `tenantId` y `urlId`. `tenantId` es el identificador de tu cuenta en FastComments.com. `urlId` es donde se asociarán los hilos de comentarios. Esto podría ser una URL de página, o un id de producto, un id de artículo, etc.

### Notificaciones de Usuario

FastComments soporta notificaciones para [muchos escenarios](https://docs.fastcomments.com/guide-notifications.html). Las notificaciones son configurables,
se pueden optar por no recibir de forma global o a nivel de notificación/comentario, y soportan suscripciones a nivel de página para que los usuarios puedan suscribirse a los hilos de una
página o artículo específico.

Por ejemplo, es posible usar Secure SSO para autenticar al usuario y luego hacer polling periódicamente en busca de notificaciones no leídas y enviárselas al usuario.

Vea [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) para ver cómo obtener y traducir las notificaciones de usuario no leídas.

### Buscador de GIFs

Por defecto, no está habilitada la selección de imágenes o gifs. Vea [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) para saber cómo
soportar subidas de imágenes y gifs. Hay un Buscador de GIFs que anonimiza búsquedas e imágenes provistas en esta librería, simplemente tienes que usarlo.

### Rendimiento

Por favor, abre un ticket con un ejemplo reproducible, incluyendo el dispositivo usado, si identificas algún problema de rendimiento. El rendimiento es una prioridad
en todas las librerías de FastComments.