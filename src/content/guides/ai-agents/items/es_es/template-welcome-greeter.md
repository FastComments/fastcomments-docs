**ID de plantilla:** `welcome_greeter`

El Welcome Greeter responde con calidez a quienes comentan por primera vez. Es la plantilla de menor riesgo (sin herramientas destructivas) y un buen primer agente para poner en producción.

### Disparadores

- **Un usuario nuevo publica su primer comentario en este sitio** (`NEW_USER_FIRST_COMMENT`).

Este evento se dispara exactamente una vez por usuario, por lo que el agente no puede entrar en bucle. Consulta [Disparador: Primer comentario de un usuario nuevo](#trigger-new-user-first-comment).

### Herramientas permitidas

- [`write_comment`](#tools-overview)

Esa es la única herramienta: el agente literalmente no puede moderar, votar, banear ni enviar mensajes directos.

### Adiciones recomendadas antes de publicarlo

- **Configura el Nombre para mostrar** con algo acogedor: "Community Bot", la mascota de tu sitio o el nombre de tu marca. El nombre para mostrar es lo que los lectores ven junto a la respuesta de bienvenida.
- **Marca "Include page title, subtitle, description, and meta tags"** en [Opciones de contexto](#context-options). Las respuestas del greeter mejoran notablemente cuando puede hacer referencia al contenido real de la página.
- **Considera las restricciones de configuración regional** si operas en varios idiomas. Una respuesta de bienvenida en el idioma equivocado es más incómoda que una respuesta omitida. Consulta [Alcance: Filtros de URL y configuración regional](#scope-url-locale).

### Por qué no se necesitan aprobaciones

El agente solo escribe comentarios nuevos y solo con un disparador de una sola vez. El peor caso: un saludo torpe. No hay ninguna acción destructiva que controlar. La mayoría de los operadores ejecutan este agente sin ninguna aprobación una vez que la ejecución de prueba se ve limpia.