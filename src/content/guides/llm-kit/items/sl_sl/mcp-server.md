FastComments poganja gostovan strežnik Model Context Protocol (MCP), tako da lahko pomočniki za pisanje kode z umetno inteligenco in agentični odjemalci neposredno kličejo FastComments API. Vsako orodje, ki ga strežnik MCP izpostavi, je samodejno ustvarjeno iz javne OpenAPI specifikacije, zato lahko MCP odjemalec naredi vse, kar lahko naredi REST API.

Končna točka je brezstanja in temelji na streamable-HTTP. Ni nobene seje, ki bi jo bilo treba vzdrževati, nobenega koraka registracije odjemalca in nobenega strežniškega stanja na odjemalca.

### Končna točka

[inline-code-attrs-start title = 'Končna točka MCP'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

Avtentikacija uporablja isti API ključ kot REST API. Prav tako lahko posredujete `tenantId` in ključ kot HTTP glave `x-tenant-id` in `x-api-key`, če vaš odjemalec podpira lastne glave.

### Prednastavljena nastavitev

Nadzorna plošča ima pomočnika za nastavitev, ki generira URL in pripravljene konfiguracijske odrezke za lepljenje za priljubljene MCP odjemalce. Pojdite na nadzorno ploščo svojega računa in odprite **Integrate -> MCP Server**, ali jo obiščite neposredno:

[inline-code-attrs-start title = 'Stran za nastavitev'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

Izberite, kateri API ključ želite uporabiti iz spustnega seznama, nato kopirajte poljuben od ustvarjenih odrezkov.

### Claude Code

Registrirajte FastComments strežnik z eno ukazno vrstico:

[inline-code-attrs-start title = 'Nastavitev Claude Code'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments 'https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY'
[inline-code-end]

Ko je registriran, za preverjanje povezave in izpis razpoložljivih orodij v seji Claude Code zaženite `/mcp`.

### Claude Desktop / Cursor

Dodajte ta blok v konfiguracijo MCP strežnikov vašega odjemalca (`claude_desktop_config.json` za Claude Desktop, `mcp.json` za Cursor):

[inline-code-attrs-start title = 'Konfiguracija MCP odjemalca'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Varnost

API ključ je vdelan v URL. Obravnavajte URL kot skrivnost: ne lepite ga v javne klepete, posnetke zaslona ali commite. Če je ključ razkrit, ga zamenjajte na strani API Keys v vaši nadzorni plošči.