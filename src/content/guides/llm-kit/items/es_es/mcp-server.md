FastComments ejecuta un servidor Model Context Protocol (MCP) alojado para que los asistentes de IA y los clientes agentes puedan llamar directamente a la API de FastComments. Cada herramienta que expone el servidor MCP se genera automáticamente a partir de la especificación OpenAPI pública, por lo que todo lo que la API REST puede hacer, un cliente MCP puede hacerlo.

El endpoint es sin estado y basado en HTTP transmisible. No hay una sesión que mantener activa y no hay estado del lado del servidor por cliente.

### Endpoint

[inline-code-attrs-start title = 'Endpoint MCP'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp
[inline-code-end]

### Conectar con OAuth

Cualquier cliente MCP que admita servidores remotos con OAuth (Claude, ChatGPT, Claude Code, Cursor y otros) puede conectarse al endpoint anterior sin configuración en FastComments. El cliente se registra a través del Registro Dinámico de Clientes o se identifica con un Documento de Metadatos de ID de Cliente, abre un navegador para que puedas iniciar sesión en FastComments y aprobar el acceso, y recibe un token vinculado a la cuenta en la que iniciaste sesión.

Los documentos de descubrimiento están en las ubicaciones estándar:

[inline-code-attrs-start title = 'Descubrimiento'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-protected-resource/mcp
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Tu usuario necesita el permiso de Administrador de API en la cuenta para aprobar una conexión. Si gestionas varias cuentas, cambia a la correcta en el panel antes de aprobar.

Un cliente puede solicitar el alcance `read`, el alcance `write`, o ambos. Un cliente que no solicite nada obtiene ambos. Las herramientas que modifican datos no se ofrecen a un token de solo lectura.

El panel tiene un asistente de configuración con fragmentos listos para copiar. Abre **Integrate -> MCP Server**, o visítalo directamente:

[inline-code-attrs-start title = 'Página de Configuración'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

### Claude Code

Registra el servidor FastComments con un solo comando, luego ejecuta `/mcp` dentro de una sesión para iniciar sesión y listar las herramientas disponibles:

[inline-code-attrs-start title = 'Configuración Claude Code'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments https://fastcomments.com/mcp
[inline-code-end]

### Cursor y otros clientes de archivo de configuración

Añade este bloque a la configuración de servidores MCP de tu cliente (`mcp.json` para Cursor). El cliente abre un navegador para iniciar sesión en el primer uso.

[inline-code-attrs-start title = 'Configuración del Cliente MCP'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "mcpServers": {
    "fastcomments": {
      "type": "http",
      "url": "https://fastcomments.com/mcp"
    }
  }
}
[inline-code-end]

### Revocar acceso

Cada conexión aprobada se lista bajo **Integrate -> Connected Apps** en el panel. Revocar una invalida todos los tokens que esa aplicación posee. Las aplicaciones se registran al conectarse y FastComments no las revisa, así que revoca cualquier cosa que no reconozcas.

### Usar el token con la API REST

El token de acceso que obtiene un cliente MCP es una credencial normal de la API de FastComments. Funciona en cada endpoint `/api/v1` como token bearer, por lo que una aplicación que se conectó a través de MCP también puede llamar directamente a la API REST:

[inline-code-attrs-start title = 'Token Bearer'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -H "Authorization: Bearer fcat_..." https://fastcomments.com/api/v1/comments
[inline-code-end]

El inquilino (tenant) está implícito en el token. Un `tenantId` aún puede enviarse pero debe coincidir. Las solicitudes `GET` necesitan el alcance `read` y todo lo demás necesita el alcance `write`.

### Conectar con una clave API

Los clientes que no pueden completar un inicio de sesión en el navegador, como servidores sin cabeza, pueden autenticarse con una clave API en su lugar. Pasa `tenantId` y `API_KEY` como parámetros de consulta, o como los encabezados HTTP `x-tenant-id` y `x-api-key` si tu cliente admite encabezados personalizados:

[inline-code-attrs-start title = 'Endpoint de Clave API'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

La página de configuración genera esta URL para cada una de tus claves API.

### Seguridad

Una URL de endpoint que contiene una clave API es un secreto: no la pegues en chats públicos, capturas de pantalla o commits. Si una clave se expone, rótala en la página de Claves API en tu panel. Los tokens OAuth no conllevan ese riesgo porque están vinculados a una sola aplicación y pueden revocarse desde Connected Apps.