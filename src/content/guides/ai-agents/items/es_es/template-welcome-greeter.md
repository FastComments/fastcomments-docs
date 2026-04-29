**ID de plantilla:** `welcome_greeter`

El Welcome Greeter responde calurosamente a quienes comentan por primera vez. Es la plantilla de menor riesgo (sin herramientas destructivas) y un buen primer agente para poner en producción.

### Prompt inicial incorporado

[inline-code-attrs-start title = 'Prompt inicial de la plantilla Welcome Greeter'; type='text' inline-code-attrs-end]
[inline-code-start]
Eres un cordial anfitrión de la comunidad. Responde a quienes comentan por primera vez con una bienvenida breve y personal. Menciona una cosa específica de su comentario para que no parezca una plantilla. Limita las respuestas a 1-2 frases. Nunca respondas a cuentas con más de 24 horas de antigüedad.
[inline-code-end]

### Desencadenantes

- **Un nuevo usuario publica su primer comentario en este sitio** (`NEW_USER_FIRST_COMMENT`).

Este evento se dispara exactamente una vez por usuario, por lo que el agente no puede entrar en un bucle. Ver [Disparador: Primer comentario de nuevo usuario](#trigger-new-user-first-comment).

### Herramientas permitidas

- [`write_comment`](#tools-overview)

Esa es la única herramienta: el agente literalmente no puede moderar, votar, banear, o enviar mensajes directos.

### Recomendaciones antes de ponerlo en producción

- **Configura el nombre para mostrar** a algo acogedor - "Community Bot", la mascota de tu sitio, o el nombre de tu marca. El nombre para mostrar es lo que los lectores ven junto a la respuesta de bienvenida.
- **Marca "Incluir título de la página, subtítulo, descripción y meta etiquetas"** en [Opciones de contexto](#context-options). Las respuestas del greeter mejoran notablemente cuando puede referenciar de qué trata la página.
- **Considera restricciones de localización** si operas en varios idiomas. Una respuesta de bienvenida en el idioma equivocado resulta más chocante que una respuesta perdida. Ver [Ámbito: Filtros de URL y Localización](#scope-url-locale).

### Por qué no se necesitan aprobaciones

El agente solo escribe nuevos comentarios y solo ante un desencadenante de única vez. En el peor de los casos: un saludo incómodo. No hay ninguna acción destructiva que restringir. La mayoría de los operadores ejecutan esto sin aprobaciones una vez que la ejecución de prueba (dry-run) parece correcta.

---