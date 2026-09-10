FastComments esegue un server Model Context Protocol (MCP) ospitato in modo che gli assistenti AI e i client agentici possano chiamare direttamente l'API di FastComments. Ogni strumento che il server MCP espone è generato automaticamente dalla specifica OpenAPI pubblica, quindi tutto ciò che l'API REST può fare, può farlo anche un client MCP.

L'endpoint è senza stato e basato su HTTP streamable. Non c'è alcuna sessione da mantenere attiva e nessuno stato lato server per client.

### Endpoint

[inline-code-attrs-start title = 'Endpoint MCP'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp
[inline-code-end]

### Connettersi con OAuth

Qualsiasi client MCP che supporta server remoti con OAuth (Claude, ChatGPT, Claude Code, Cursor e altri) può connettersi all'endpoint sopra senza configurazione da parte di FastComments. Il client si registra tramite Dynamic Client Registration o si identifica con un Client ID Metadata Document, apre un browser così puoi accedere a FastComments e approvare l'accesso, e riceve un token associato all'account con cui sei connesso.

I documenti di scoperta si trovano nelle posizioni standard:

[inline-code-attrs-start title = 'Scoperta'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-protected-resource/mcp
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Il tuo utente ha bisogno del permesso API Admin sull'account per approvare una connessione. Se gestisci più account, passa a quello corretto nella dashboard prima di approvare.

Un client può richiedere lo scope `read`, lo scope `write`, o entrambi. Un client che non richiede nulla ottiene entrambi. Gli strumenti che modificano i dati non sono offerti a un token in sola lettura.

La dashboard ha un assistente di configurazione con snippet pronti da incollare. Apri **Integrate -> MCP Server**, o visitalo direttamente:

[inline-code-attrs-start title = 'Pagina di Configurazione'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

### Claude Code

Registra il server FastComments con un solo comando, poi esegui `/mcp` all'interno di una sessione per accedere e elencare gli strumenti disponibili:

[inline-code-attrs-start title = 'Configurazione Claude Code'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments https://fastcomments.com/mcp
[inline-code-end]

### Cursor e altri client basati su file di configurazione

Aggiungi questo blocco alla configurazione dei server MCP del tuo client (`mcp.json` per Cursor). Il client apre un browser per accedere al primo utilizzo.

[inline-code-attrs-start title = 'Configurazione Client MCP'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Revocare l'accesso

Ogni connessione approvata è elencata sotto **Integrate -> Connected Apps** nella dashboard. Revocare una connessione invalida tutti i token che quell'applicazione possiede. Le applicazioni si registrano quando si connettono e FastComments non le revisiona, quindi revoca tutto ciò che non riconosci.

### Usare il token con l'API REST

Il token di accesso che un client MCP ottiene è una credenziale API FastComments standard. Funziona su ogni endpoint `/api/v1` come token bearer, quindi un'applicazione connessa tramite MCP può anche chiamare direttamente l'API REST:

[inline-code-attrs-start title = 'Token Bearer'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -H "Authorization: Bearer fcat_..." https://fastcomments.com/api/v1/comments
[inline-code-end]

Il tenant è implicito nel token. Un `tenantId` può ancora essere passato ma deve corrispondere. Le richieste `GET` richiedono lo scope `read` e tutto il resto richiede lo scope `write`.

### Connettersi con una chiave API

I client che non possono completare l'accesso tramite browser, come i server headless, possono autenticarsi invece con una chiave API. Passa `tenantId` e `API_KEY` come parametri di query, o come intestazioni HTTP `x-tenant-id` e `x-api-key` se il tuo client supporta intestazioni personalizzate:

[inline-code-attrs-start title = 'Endpoint Chiave API'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

La pagina di configurazione genera questo URL per ciascuna delle tue chiavi API.

### Sicurezza

Un URL endpoint che contiene una chiave API è un segreto: non incollarlo in chat pubbliche, screenshot o commit. Se una chiave viene esposta, ruotala nella pagina Chiavi API della tua dashboard. I token OAuth non comportano tale rischio perché sono legati a un'applicazione e possono essere revocati da Connected Apps.