FastComments betreibt einen gehosteten Model Context Protocol (MCP)-Server, sodass KI‑Assistenten und agentische Clients die FastComments‑API direkt aufrufen können. Jedes Tool, das der MCP‑Server bereitstellt, wird automatisch aus dem öffentlichen OpenAPI‑Spec generiert, sodass alles, was die REST‑API kann, auch ein MCP‑Client ausführen kann.

Der Endpunkt ist zustandslos und streamable‑HTTP‑basiert. Es gibt keine Sitzung, die aufrechterhalten werden muss, und keinen serverseitigen Zustand pro Client.

### Endpunkt

[inline-code-attrs-start title = 'MCP-Endpunkt'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp
[inline-code-end]

### Verbindung mit OAuth

Jeder MCP‑Client, der Remote‑Server mit OAuth unterstützt (Claude, ChatGPT, Claude Code, Cursor und andere), kann sich ohne Einrichtung auf der FastComments‑Seite mit dem oben genannten Endpunkt verbinden. Der Client registriert sich über die dynamische Client‑Registrierung oder identifiziert sich mit einem Client‑ID‑Metadaten‑Dokument, öffnet einen Browser, sodass Sie sich bei FastComments anmelden und den Zugriff genehmigen können, und erhält ein Token, das an das Konto gebunden ist, bei dem Sie angemeldet waren.

Die Entdeckungsdokumente befinden sich an den Standardorten:

[inline-code-attrs-start title = 'Entdeckung'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-protected-resource/mcp
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Ihr Benutzer benötigt die API‑Admin‑Berechtigung für das Konto, um eine Verbindung zu genehmigen. Wenn Sie mehrere Konten verwalten, wechseln Sie im Dashboard zum richtigen Konto, bevor Sie genehmigen.

Ein Client kann den `read`‑Scope, den `write`‑Scope oder beide anfordern. Ein Client, der nichts anfordert, erhält beide. Werkzeuge, die Daten ändern, werden einem reinen Lese‑Token nicht angeboten.

Das Dashboard enthält einen Einrichtungs‑Assistenten mit sofort einfügbaren Snippets. Öffnen Sie **Integrate → MCP Server** oder rufen Sie die Seite direkt auf:

[inline-code-attrs-start title = 'Setup-Seite'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

### Claude Code

Registrieren Sie den FastComments‑Server mit einem Befehl und führen Sie anschließend `/mcp` innerhalb einer Sitzung aus, um sich anzumelden und die verfügbaren Werkzeuge aufzulisten:

[inline-code-attrs-start title = 'Claude-Code-Einrichtung'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments https://fastcomments.com/mcp
[inline-code-end]

### Cursor und andere Konfigurationsdatei‑Clients

Fügen Sie diesen Block zur MCP‑Server‑Konfiguration Ihres Clients hinzu (`mcp.json` für Cursor). Der Client öffnet beim ersten Gebrauch einen Browser, um sich anzumelden.

[inline-code-attrs-start title = 'MCP-Client-Konfiguration'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Zugriff widerrufen

Jede genehmigte Verbindung wird im Dashboard unter **Integrate → Connected Apps** angezeigt. Das Widerrufen einer Verbindung macht jedes Token, das diese Anwendung besitzt, ungültig. Anwendungen registrieren sich selbst, wenn sie sich verbinden, und FastComments prüft sie nicht, daher sollten Sie alles widerrufen, was Sie nicht erkennen.

### Verwendung des Tokens mit der REST‑API

Das Zugriffstoken, das ein MCP‑Client erhält, ist ein reguläres FastComments‑API‑Credential. Es funktioniert auf jedem `/api/v1`‑Endpunkt als Bearer‑Token, sodass eine Anwendung, die über MCP verbunden ist, die REST‑API ebenfalls direkt aufrufen kann:

[inline-code-attrs-start title = 'Bearer-Token'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -H "Authorization: Bearer fcat_..." https://fastcomments.com/api/v1/comments
[inline-code-end]

Der Mandant wird durch das Token impliziert. Ein `tenantId` kann weiterhin übergeben werden, muss jedoch übereinstimmen. `GET`‑Anfragen benötigen den `read`‑Scope und alle anderen benötigen den `write`‑Scope.

### Verbindung mit einem API‑Schlüssel

Clients, die keinen Browser‑Login durchführen können, wie z. B. headless‑Server, können sich stattdessen mit einem API‑Schlüssel authentifizieren. Übergeben Sie `tenantId` und `API_KEY` als Abfrageparameter oder als die HTTP‑Header `x-tenant-id` und `x-api-key`, falls Ihr Client benutzerdefinierte Header unterstützt:

[inline-code-attrs-start title = 'API-Schlüssel-Endpunkt'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

Die Einrichtungsseite erzeugt diese URL für jeden Ihrer API‑Schlüssel.

### Sicherheit

Eine Endpunkt‑URL, die einen API‑Schlüssel enthält, ist ein Geheimnis: Fügen Sie sie nicht in öffentlichen Chats, Screenshots oder Commits ein. Wenn ein Schlüssel offengelegt wird, rotieren Sie ihn auf der Seite API‑Keys in Ihrem Dashboard. OAuth‑Tokens bergen dieses Risiko nicht, da sie an eine Anwendung gebunden sind und über Connected Apps widerrufen werden können.

---