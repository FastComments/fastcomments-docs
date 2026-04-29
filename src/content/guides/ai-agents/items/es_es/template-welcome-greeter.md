**Template ID:** `welcome_greeter`

El Salutador de Bienvenida responde con calidez a los comentaristas primerizos. Es la plantilla de menor riesgo (sin herramientas destructivas) y un buen primer agente para poner en producción.

### Triggers

- **New user posts their first comment on this site** (`NEW_USER_FIRST_COMMENT`).

Este evento se activa exactamente una vez por usuario, por lo que el agente no puede entrar en bucle. Vea [Disparador: Nuevo Usuario Primer Comentario](#trigger-new-user-first-comment).

### Allowed tools

- [`write_comment`](#tools-overview)

Esa es la única herramienta: el agente literalmente no puede moderar, votar, prohibir ni enviar mensajes directos.

### Recommended additions before going live

- **Set the Display name** a algo acogedor: "Community Bot", la mascota de su sitio o el nombre de su marca. El nombre que se muestra es lo que los lectores ven adjunto a la respuesta de bienvenida.
- **Tick "Include page title, subtitle, description, and meta tags"** en [Opciones de contexto](#context-options). Las respuestas del saludador mejoran notablemente cuando puede hacer referencia a lo que realmente trata la página.
- **Consider locale restrictions** si opera en varios idiomas. Una respuesta de bienvenida en el idioma equivocado resulta más chocante que una respuesta omitida. Véase [Alcance: Filtros de URL y de configuración regional](#scope-url-locale).

### Why no approvals are needed

El agente solo escribe nuevos comentarios y solo ante un disparador de una sola vez. En el peor de los casos: un saludo incómodo. No hay ninguna acción destructiva que requiera autorización. La mayoría de los operadores ejecutan este agente sin aprobaciones una vez que la ejecución en modo de prueba parece correcta.

---