FastComments betreibt einen gehosteten Model Context Protocol (MCP) Server, damit KI-Coding-Assistenten und agentische Clients direkt die FastComments-API aufrufen können. Jedes Tool, das der MCP-Server bereitstellt, wird automatisch aus dem öffentlichen OpenAPI-Spec generiert, sodass alles, was die REST-API kann, auch ein MCP-Client kann.

Der Endpunkt ist zustandslos und basiert auf streamfähigem HTTP. Es gibt keine Sitzung, die aufrechterhalten werden muss, keinen Registrierungsschritt für Clients und keinen serverseitigen Zustand pro Client.

### Endpunkt

[inline-code-attrs-start title = 'MCP-Endpunkt'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

Die Authentifizierung verwendet denselben API-Schlüssel wie die REST-API. Sie können auch `tenantId` und den Schlüssel als die HTTP-Header `x-tenant-id` und `x-api-key` übergeben, wenn Ihr Client benutzerdefinierte Header unterstützt.

### Vorkonfigurierte Einrichtung

Das Dashboard bietet einen Einrichtungsassistenten, der die URL und einsatzbereite Konfigurationsschnipsel für gängige MCP-Clients generiert. Gehen Sie zu Ihrem Konto-Dashboard und öffnen Sie **Integrate -> MCP Server**, oder rufen Sie es direkt auf:

[inline-code-attrs-start title = 'Einrichtungsseite'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

Wählen Sie im Dropdown-Menü den zu verwendenden API-Schlüssel aus und kopieren Sie dann einen der generierten Schnipsel.

### Claude Code

Registrieren Sie den FastComments-Server mit einem Befehl:

[inline-code-attrs-start title = 'Claude Code Einrichtung'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments 'https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY'
[inline-code-end]

Nachdem er registriert ist, führen Sie `/mcp` in einer Claude Code-Sitzung aus, um die Verbindung zu bestätigen und die verfügbaren Tools aufzulisten.

### Claude Desktop / Cursor

Fügen Sie diesen Block der MCP-Server-Konfiguration Ihres Clients hinzu (`claude_desktop_config.json` für Claude Desktop, `mcp.json` für Cursor):

[inline-code-attrs-start title = 'MCP-Client-Konfiguration'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Sicherheit

Der API-Schlüssel ist in der URL eingebettet. Behandeln Sie die URL wie ein Geheimnis: Fügen Sie sie nicht in öffentliche Chats, Screenshots oder Commits ein. Falls ein Schlüssel offengelegt wurde, tauschen Sie ihn auf der Seite 'API Keys' in Ihrem Dashboard aus.