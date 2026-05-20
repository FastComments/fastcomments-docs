---
FastComments ejecuta un servidor alojado de Model Context Protocol (MCP) para que asistentes de codificación con IA y clientes agentivos puedan llamar a la API de FastComments directamente. Todas las herramientas que expone el servidor MCP se generan automáticamente a partir de la especificación pública OpenAPI, así que todo lo que la API REST puede hacer, un cliente MCP puede hacerlo.

El endpoint es sin estado y está basado en HTTP con streaming. No hay sesión que mantener viva, no hay un paso de registro del cliente, y no hay estado del lado del servidor por cliente.

### Punto final

[inline-code-attrs-start title = 'Punto final MCP'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

La autenticación usa la misma clave API que la API REST. También puedes pasar `tenantId` y la clave como `x-tenant-id` y `x-api-key` cabeceras HTTP si tu cliente admite cabeceras personalizadas.

### Configuración prellenada

El panel de control tiene un asistente de configuración que genera la URL y fragmentos de configuración listos para pegar para clientes MCP populares. Ve al panel de control de tu cuenta y abre **Integrate -> MCP Server**, o visítalo directamente:

[inline-code-attrs-start title = 'Página de configuración'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

Selecciona cuál clave API usar desde el desplegable, y luego copia cualquiera de los fragmentos generados.

### Claude Code

Registra el servidor de FastComments con un comando:

[inline-code-attrs-start title = 'Configuración de Claude Code'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments 'https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY'
[inline-code-end]

Después de registrarlo, ejecuta `/mcp` dentro de una sesión de Claude Code para confirmar la conexión y listar las herramientas disponibles.

### Claude Desktop / Cursor

Añade este bloque a la configuración de servidores MCP de tu cliente (`claude_desktop_config.json` para Claude Desktop, `mcp.json` para Cursor):

[inline-code-attrs-start title = 'Configuración del cliente MCP'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "mcpServers": {
    "fastcomments": {
      "type": "http",
      "url": "https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY"
    }
  }
}
[inline-code-end]

### Seguridad

La clave API está incrustada en la URL. Trata la URL como un secreto: no la pegues en chats públicos, capturas de pantalla, ni commits. Si una clave se expone, rótala en la página de Claves API en tu panel de control.

---