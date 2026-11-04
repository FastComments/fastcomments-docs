FastComments driver en hostet Model Context Protocol (MCP)-server, så AI-assistenter og agentiske klienter kan kalde FastComments API direkte. Hvert værktøj, som MCP-serveren eksponerer, er automatisk genereret fra den offentlige OpenAPI-spec, så alt hvad REST API'en kan, kan en MCP-klient også gøre.

Endpointet er stateless og baseret på streamable-HTTP. Der er ingen session, der skal holdes i live, og ingen server-side tilstand pr. klient.

### Endpoint

[inline-code-attrs-start title = 'MCP Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp
[inline-code-end]

### Forbind med OAuth

Enhver MCP-klient, der understøtter fjernservere med OAuth (Claude, ChatGPT, Claude Code, Cursor og andre), kan forbinde til endpointet ovenfor uden opsætning på FastComments-siden. Klienten registrerer sig selv via Dynamic Client Registration eller identificerer sig med et Client ID Metadata Document, åbner en browser, så du kan logge ind på FastComments og godkende adgangen, og modtager en token bundet til den konto, du var logget ind på.

Discovery-dokumenterne findes på standardplaceringerne:

[inline-code-attrs-start title = 'Discovery'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-protected-resource/mcp
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Din bruger skal have API Admin-tilladelsen på kontoen for at godkende en forbindelse. Hvis du administrerer flere konti, så skift til den rigtige i dashboardet, før du godkender.

En klient kan anmode om `read`-scope, `write`-scope eller begge. En klient, der ikke anmoder om noget, får begge. Værktøjer, der ændrer data, tilbydes ikke til en read-only token.

Dashboardet har en opsætningshjælper med klar-til-indsættelse kodestykker. Åbn **Integrate -> MCP Server**, eller besøg den direkte:

[inline-code-attrs-start title = 'Opsætningsside'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

### Claude Code

Registrér FastComments-serveren med én kommando, og kør derefter `/mcp` inde i en session for at logge ind og se de tilgængelige værktøjer:

[inline-code-attrs-start title = 'Claude Code Opsætning'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments https://fastcomments.com/mcp
[inline-code-end]

### Cursor og andre config-fil-klienter

Tilføj denne blok til din klients MCP-servers-konfiguration (`mcp.json` for Cursor). Klienten åbner en browser til login ved første brug.

[inline-code-attrs-start title = 'MCP-klientkonfiguration'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Tilbagekaldelse af adgang

Enhver godkendt forbindelse er listet under **Integrate -> Connected Apps** i dashboardet. Tilbagekaldelse af en forbindelse ugyldiggør alle tokens, som applikationen holder. Applikationer registrerer sig selv, når de forbinder, og FastComments gennemgår dem ikke, så tilbagekald alt, du ikke genkender.

### Brug af token med REST API'en

Den access token, som en MCP-klient opnår, er en almindelig FastComments API-credential. Den virker på alle `/api/v1`-endpoints som en bearer token, så en applikation, der har forbundet via MCP, også kan kalde REST API'en direkte:

[inline-code-attrs-start title = 'Bearer Token'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -H "Authorization: Bearer fcat_..." https://fastcomments.com/api/v1/comments
[inline-code-end]

Tenanten er underforstået af tokenen. En `tenantId` kan stadig sendes med, men den skal matche. `GET`-requests kræver `read`-scope, og alt andet kræver `write`-scope.

### Forbind med en API-nøgle

Klienter, der ikke kan gennemføre et browser-login, såsom headless-servere, kan i stedet autentificere med en API-nøgle. Send `tenantId` og `API_KEY` som query-parametre, eller som `x-tenant-id`- og `x-api-key`-HTTP-headers, hvis din klient understøtter custom headers:

[inline-code-attrs-start title = 'API-nøgle Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

Opsætningssiden genererer denne URL for hver af dine API-nøgler.

### Sikkerhed

En endpoint-URL, der indeholder en API-nøgle, er en hemmelighed: indsæt den ikke i offentlige chats, screenshots eller commits. Hvis en nøgle er blevet eksponeret, så rotér den på API Keys-siden i dit dashboard. OAuth-tokens bærer ikke denne risiko, da de er bundet til én applikation og kan tilbagekaldes fra Connected Apps.