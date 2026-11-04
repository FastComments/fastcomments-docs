Comentarios en vivo con hilos, avatares, respuestas anidadas, votos y el compositor de texto enriquecido integrado, además de un tema oscuro y un preset de chat en vivo (mostrado aquí renderizado vía `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Comentarios en vivo</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-light.png" width="260" alt="Comentarios en vivo, tema claro"/></td>
    <td align="center"><b>Tema oscuro</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-dark.png" width="260" alt="Comentarios en vivo, tema oscuro"/></td>
    <td align="center"><b>Chat en vivo</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-chat.png" width="260" alt="Preset de chat en vivo"/></td>
  </tr>
</table>

### Editor de texto enriquecido

Esta librería utiliza [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched-html) para la edición de texto enriquecido, que ofrece una potente experiencia de edición WYSIWYG. El mismo editor funciona en iOS, Android y la web (vía `react-native-web`), por lo que el compositor se comporta de manera consistente en todas las plataformas con una única implementación.

`react-native-enriched` requiere la Nueva Arquitectura de React Native (Fabric) en nativo (predeterminada desde RN 0.76, opcional en RN 0.72-0.75), y un bundler que resuelva las condiciones de `exports` del paquete. Este SDK se desarrolla y prueba con RN 0.81 / React 19. El mismo editor también funciona en la web a través de `react-native-web`; la compilación web del editor enriched todavía está marcada como experimental upstream.

### Widgets

El SDK incluye tres widgets, en paralelo con el FastComments Android SDK:

- `FastCommentsLiveCommenting` - comentarios en hilo con votos, respuestas, paginación, menciones, notificaciones y actualizaciones en vivo.
- `FastCommentsLiveChat` - un preset de chat sobre el mismo motor: mensajes cronológicos con los nuevos en la parte inferior, el compositor debajo de la lista, una franja de encabezado en vivo (punto de conexión + conteo de usuarios), historial infinito cargado al desplazarse hacia arriba, desplazamiento automático a los nuevos mensajes, sin votos ni hilos de respuesta. Cada preset puede sobrescribirse mediante `config`.
- `FastCommentsFeed` - un feed social con compositor de publicaciones, medios, reacciones, seguidores y banners de nuevas publicaciones en vivo.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Tematización

La apariencia predeterminada se genera a partir de un conjunto de tokens de diseño semánticos (`FastCommentsTheme`): colores, espaciado, radios, tamaños de fuente, grosores de fuente y tamaños de avatar. Pase anulaciones parciales de tokens (tipadas como `FastCommentsThemeOverrides`) a través de la prop `theme` en cualquier widget y todo el árbol de estilos se restiliza de forma consistente:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

El modo oscuro está a un conjunto de tokens de distancia:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

La prop `styles` todavía acepta un árbol `IFastCommentsStyles` crudo para un control quirúrgico. Cuando se proporcionan tanto `theme` como `styles`, los estilos explícitos ganan sobre el árbol tematizado; cuando solo se proporciona `styles`, reemplaza por completo los valores predeterminados (el comportamiento original, por lo que las integraciones y skins existentes no se ven afectadas). `setupDarkModeSkin` está obsoleto en favor de la prop `theme`.

### Opciones de configuración

Esta librería aims a soportar todas las opciones de configuración definidas en [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), igual que la implementación web.

Además de esas, React Native añade algunas opciones específicas del SDK vía `FastCommentsRNConfig`:

- `hideTopBar` - oculta la franja de usuario conectado / campana de notificaciones mostrada sobre el compositor.
- `usePressToEdit` - mantén presionado un comentario para abrir su menú.
- `disableDownVoting` - oculta los botones de voto negativo.
- `renderCommentInline` - renderiza la información del comentarista dentro del mismo bloque HTML que el contenido del comentario.
- `renderLikesToRight` - mueve el área de votos/me gusta a la derecha del comentario en lugar de debajo.
- `renderDateBelowComment` - renderiza la fecha debajo del comentario.
- `showLiveStatus` - muestra la franja de encabezado estilo chat "En vivo" + conteo de usuarios sobre los comentarios.
- `useInlineSubmitButton` - renderiza el botón de envío como un icono dentro del compositor.
- `countAboveToggle` - con `useShowCommentsToggle`, cuántos comentarios se renderizan sobre el interruptor "Mostrar comentarios".
- `preserveFeedScrollPosition` - `FastCommentsFeed` recuerda su posición de desplazamiento entre desmontajes/remontajes (predeterminado true).

### Conceptos de FastComments

Los conceptos principales a tener en cuenta para comenzar son `tenantId` y `urlId`. `tenantId` es el identificador de tu cuenta de FastComments.com. `urlId` es a dónde se vincularán los hilos de comentarios. Puede ser la URL de una página, o un id de producto, un id de artículo, etc.

### Localización

Todo el texto visible para el usuario en estos widgets (etiquetas de botones, placeholders, estados vacíos, fechas relativas como "hace 5 minutos", mensajes de error, etc.) es **controlado por el servidor**. Los componentes no tienen cadenas en inglés codificadas de forma fija; renderizan las traducciones que FastComments sirve para la configuración regional solicitada.

Para solicitar una configuración regional, establece `locale` en tu configuración:

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, etc.
};
```

Cuando no se establece `locale`, FastComments sirve el idioma predeterminado del tenant.

**Editar el texto:** las traducciones se gestionan en tu panel de FastComments, no en este SDK. Para cambiar la redacción, sobrescribir el texto predeterminado o añadir un idioma, edita las traducciones de tu cuenta en el panel - el cambio es recogido por los widgets automáticamente sin necesidad de publicar la app. El SDK no incluye respaldos en inglés, por lo que cualquier clave que dejes en blanco en el panel se renderiza vacía; mantén las claves pobladas para cada configuración regional que soportes.

### Notificaciones de usuario

FastComments soporta notificaciones para [muchos escenarios](https://docs.fastcomments.com/guide-notifications.html). Las notificaciones son configurables, se pueden desactivar globalmente o a nivel de notificación/comentario, y soporta suscripciones a nivel de página para que los usuarios puedan suscribirse a los hilos de una página o artículo específico.

Por ejemplo, es posible usar Secure SSO para autenticar al usuario y luego sondear periódicamente las notificaciones no leídas y empujarlas al usuario.

Consulta [el ejemplo AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) para saber cómo obtener y traducir las notificaciones de usuario no leídas.

### Navegador de Gifs

Por defecto, no hay selección de imágenes ni gifs habilitada. Consulta [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) para saber cómo soportar la subida de imágenes y gifs. Hay un Navegador de Gifs que anonimiza las búsquedas y las imágenes proporcionadas en esta librería; solo tienes que usarlo.

### Rendimiento

Por favor, abre un ticket con un ejemplo para reproducir, incluyendo el dispositivo utilizado, si identificas algún problema de rendimiento. El rendimiento es un ciudadano de primera clase en todas las librerías de FastComments.