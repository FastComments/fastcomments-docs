Ahora vamos a generar tu código personalizado de FastComments. Usa el asistente a continuación para configurar cómo quieres que FastComments funcione en tu sitio de GoHighLevel:

[snippet id="gohighlevel-wizard"]

### Diferentes tipos de cuadro de comentarios

Puedes configurar la línea `TYPE = 'commenting'` para cambiar el producto usado (por ejemplo, puedes cambiarla a `live` para chat en streaming o `collab` para chat colaborativo).

### Colocar el cuadro de comentarios donde quieras

Supongamos que quieres colocar cuadros de comentarios en partes específicas de la página y no en las ubicaciones predeterminadas.
Cambia esta línea:

    const TARGET_ELEMENT_ID = ''; // set to use target div mode

A:

    const TARGET_ELEMENT_ID = 'fc_box'; // set to use target div mode

Entonces, en el editor de GHL, haz clic en el botón "code" y añade donde quieres que vayan los comentarios:

[inline-code-attrs-start title = 'Div de FastComments de GoHighLevel'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="commenting"
  urlid="custom-chat-id"
></div>
[inline-code-end]

### Diferente tipo de cuadro de comentarios por página

Supongamos que quieres que los usuarios resalten y discutan fragmentos de texto, o que usen en su lugar la interfaz de chat en streaming.

Primero sigue los pasos anteriores en "Colocar el cuadro de comentarios donde quieras".

Fíjate que en ese pequeño fragmento está `type="commenting"`.

Si quieres habilitar, por ejemplo, el chat colaborativo, cambia type a `type="collab"`.

### Mostrar solo en páginas específicas

Si no estableces no estableces `TARGET_ELEMENT_ID`, en su lugar puedes configurar la variable `VALID_PATTERNS`, para definir en qué rutas de URL deben mostrarse los comentarios. Por defecto, se mostrarán en páginas que contengan `/post` en la URL.

### Configuración del chat colaborativo

Puedes indicar al chat colaborativo que solo añada funcionalidad colaborativa alrededor del HTML dentro de un área específica; por ejemplo, supongamos que añades el código de pie de página arriba y luego agregas este div en el contenido del post/página para habilitar el chat colaborativo:

[inline-code-attrs-start title = 'Chat colaborativo con contenido especificado'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="collab"
  urlid="custom-chat-id"
><p>This content will have collab chat!</p></div>
[inline-code-end]

Entonces el elemento párrafo dentro del `<div>` tendrá el chat colaborativo habilitado, y nada más en la página. Si no pones ningún contenido en el `<div>`, entonces habilitará el chat colaborativo en todo el cuerpo de la publicación.

---