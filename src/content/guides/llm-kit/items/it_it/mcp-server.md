FastComments gestisce un server ospitato del Model Context Protocol (MCP) in modo che gli assistenti di programmazione AI e i client agentici possano chiamare direttamente l'API di FastComments. Ogni strumento esposto dal server MCP è generato automaticamente dalla specifica OpenAPI pubblica, quindi tutto ciò che l'API REST può fare, un client MCP può farlo.

L'endpoint è senza stato e basato su HTTP con supporto allo streaming. Non ci sono sessioni da mantenere attive, nessun passaggio di registrazione del client e nessuno stato lato server per singolo client.

### Endpoint

[inline-code-attrs-start title = 'Endpoint MCP'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

L'autenticazione usa la stessa API key dell'API REST. Puoi anche passare `tenantId` e la chiave come intestazioni HTTP `x-tenant-id` e `x-api-key` se il tuo client supporta intestazioni personalizzate.

### Configurazione precompilata

La dashboard include un assistente di configurazione che genera l'URL e frammenti di configurazione pronti da incollare per i client MCP più diffusi. Vai alla dashboard del tuo account e apri **Integrate -> MCP Server**, oppure visitala direttamente:

[inline-code-attrs-start title = 'Pagina di configurazione'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

Scegli quale API key usare dal menu a discesa, quindi copia uno degli snippet generati.

### Claude Code

Registra il server FastComments con un solo comando:

[inline-code-attrs-start title = 'Configurazione Claude Code'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments 'https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY'
[inline-code-end]

Dopo la registrazione, esegui `/mcp` all'interno di una sessione Claude Code per confermare la connessione e elencare gli strumenti disponibili.

### Claude Desktop / Cursor

Aggiungi questo blocco alla configurazione dei server MCP del tuo client (`claude_desktop_config.json` per Claude Desktop, `mcp.json` per Cursor):

[inline-code-attrs-start title = 'Configurazione client MCP'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Sicurezza

La API key è incorporata nell'URL. Tratta l'URL come un segreto: non incollarlo in chat pubbliche, screenshot o commit. Se una chiave viene esposta, ruotala nella pagina API Keys della tua dashboard.