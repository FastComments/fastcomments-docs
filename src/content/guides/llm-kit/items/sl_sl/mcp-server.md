FastComments poganja gostovan strežnik Model Context Protocol (MCP), tako da AI asistenti in agentni odjemalci lahko neposredno kličijo FastComments API. Vsako orodje, ki ga strežnik MCP razkrije, je samodejno ustvarjeno iz javne specifikacije OpenAPI, zato lahko MCP odjemalec naredi vse, kar lahko REST API.

Končna točka je brez stanja in temelji na streamable‑HTTP. Ni seje, ki bi jo bilo treba ohranjati, in ni strežniškega stanja na odjemalca.

### Končna točka

[inline-code-attrs-start title = 'MCP končna točka'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp
[inline-code-end]

### Povezava z OAuth

Katerikoli MCP odjemalec, ki podpira oddaljene strežnike z OAuth (Claude, ChatGPT, Claude Code, Cursor in drugi), se lahko poveže z zgornjo končno točko brez dodatne nastavitve na strani FastComments. Odjemalec se registrira prek dinamične registracije odjemalcev ali se identificira z dokumentom metapodatkov ID‑ja odjemalca, odpre brskalnik, da se lahko prijavite v FastComments in odobrite dostop, ter prejme žeton, vezan na račun, v katerega ste se prijavili.

Dokumenti za odkrivanje so na standardnih lokacijah:

[inline-code-attrs-start title = 'Odkrivanje'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-protected-resource/mcp
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Vaš uporabnik potrebuje dovoljenje API Admin na računu, da odobri povezavo. Če upravljate z več računi, preklopite na pravi v nadzorni plošči, preden odobrite.

Odjemalec lahko zahteva obseg `read`, obseg `write` ali oba. Odjemalec, ki ne zahteva ničesar, prejme oba. Orodja, ki spreminjajo podatke, niso na voljo za žeton samo za branje.

Nadzorna plošča ima pomočnika za nastavitve s pripravljenimi izrezki za lepljenje. Odprite **Integrate -> MCP Server**, ali obiščite neposredno:

[inline-code-attrs-start title = 'Stran nastavitve'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

### Claude Code

Registrirajte FastComments strežnik z enim ukazom, nato zaženite `/mcp` v seji, da se prijavite in izpišete razpoložljiva orodja:

[inline-code-attrs-start title = 'Nastavitev Claude Code'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments https://fastcomments.com/mcp
[inline-code-end]

### Cursor in drugi odjemalci s konfiguracijsko datoteko

Dodajte ta blok v konfiguracijo MCP strežnikov vašega odjemalca (`mcp.json` za Cursor). Odjemalec odpre brskalnik za prijavo ob prvi uporabi.

[inline-code-attrs-start title = 'Konfiguracija MCP odjemalca'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Preklic dostopa

Vsaka odobrena povezava je navedena pod **Integrate -> Connected Apps** v nadzorni plošči. Preklic ene povezave razveljavi vsak žeton, ki ga aplikacija ima. Aplikacije se registrirajo, ko se povežejo, FastComments jih ne pregleda, zato prekličite vse, kar ne prepoznate.

### Uporaba žetona z REST API

Dostopni žeton, ki ga pridobi MCP odjemalec, je običajno poverilnico FastComments API. Deluje na vsaki končni točki `/api/v1` kot žeton nosilca, zato lahko aplikacija, ki se je povezana prek MCP, neposredno kliče tudi REST API:

[inline-code-attrs-start title = 'Žeton nosilca'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -H "Authorization: Bearer fcat_..." https://fastcomments.com/api/v1/comments
[inline-code-end]

Najemnik je impliciran v žetonu. `tenantId` je še vedno mogoče posredovati, vendar se mora ujemati. `GET` zahteve potrebujejo obseg `read`, vse ostalo pa obseg `write`.

### Povezava z API ključem

Odjemalci, ki ne morejo dokončati prijave v brskalniku, kot so brezglave strežniki, se lahko namesto tega avtenticirajo z API ključem. Posredujte `tenantId` in `API_KEY` kot parametra poizvedbe ali kot HTTP glave `x-tenant-id` in `x-api-key`, če vaš odjemalec podpira prilagojene glave:

[inline-code-attrs-start title = 'Končna točka API ključa'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

Stran za nastavitve ustvari ta URL za vsak vaš API ključ.

### Varnost

URL končne točke, ki vsebuje API ključ, je skrivnost: ne prilepite ga v javne klepete, posnetke zaslona ali commite. Če je ključ izpostavljen, ga zamenjajte na strani API ključev v vaši nadzorni plošči. OAuth žetoni ne predstavljajo takšnega tveganja, ker so vezani na eno aplikacijo in jih je mogoče preklicati v Connected Apps.