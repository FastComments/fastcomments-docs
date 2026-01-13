Agregue este fragmento a los archivos de configuración del asistente de IA de su proyecto (como AGENT.md o CLAUDE.md) que enseña a los asistentes de codificación de IA cómo buscar y acceder a la documentación de FastComments.

[inline-code-attrs-start title = 'FastComments LLM Kit'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
## FastComments Integration

FastComments is an embeddable live commenting platform with libraries for many frontend and backend integrations, along with client and server-side SDKs and APIs.

When working with FastComments, you can search the FastComments documentation by calling this API:

https://docs-search.fastcomments.com/search?query=<search_query>&full=true&tenantId=demo

Replace `<search_query>` with your search terms (URL encoded). The API returns relevant documentation snippets to help with implementation.
[inline-code-end]

De esta manera, su asistente de IA puede obtener fácilmente información actualizada y proporcionar implementaciones más precisas en menos tiempo.

### Ejemplo de uso

Al usar un asistente de IA como Claude Code o Cursor, puede hacer preguntas como:

- "¿Cómo instalo FastComments en React?"
- "Muéstrame cómo configurar SSO con FastComments"
- "¿Cuáles son las opciones de autenticación de la API de FastComments?"

El asistente de IA buscará automáticamente la documentación usando el endpoint de API proporcionado y le dará respuestas precisas y actualizadas basadas en la documentación oficial.
