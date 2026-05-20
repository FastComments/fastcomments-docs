FastComments uruchamia hostowany serwer Model Context Protocol (MCP), dzięki czemu asystenci kodowania AI i klienci agentowi mogą wywoływać bezpośrednio API FastComments. Każde narzędzie, które serwer MCP udostępnia, jest automatycznie generowane z publicznej specyfikacji OpenAPI, więc wszystko, co potrafi REST API, potrafi też klient MCP.

Punkt końcowy jest bezstanowy i oparty na strumieniowym HTTP. Nie ma sesji do utrzymania, kroku rejestracji klienta ani stanu po stronie serwera dla każdego klienta.

### Endpoint

[inline-code-attrs-start title = 'Punkt końcowy MCP'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

Uwierzytelnianie używa tego samego klucza API co REST API. Możesz również przekazać `tenantId` i klucz jako nagłówki HTTP `x-tenant-id` i `x-api-key`, jeśli twój klient obsługuje niestandardowe nagłówki.

### Wstępna konfiguracja

Panel ma kreatora konfiguracji, który generuje URL i gotowe do wklejenia fragmenty konfiguracji dla popularnych klientów MCP. Przejdź do pulpitu konta i otwórz **Integracje -> Serwer MCP**, lub odwiedź to bezpośrednio:

[inline-code-attrs-start title = 'Strona konfiguracji'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

Wybierz, którego klucza API chcesz użyć z rozwijanego menu, a następnie skopiuj dowolny z wygenerowanych fragmentów.

### Claude Code

Zarejestruj serwer FastComments jednym poleceniem:

[inline-code-attrs-start title = 'Konfiguracja Claude Code'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments 'https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY'
[inline-code-end]

Po rejestracji uruchom `/mcp` podczas sesji Claude Code, aby potwierdzić połączenie i wyświetlić listę dostępnych narzędzi.

### Claude Desktop / Cursor

Dodaj ten blok do konfiguracji serwerów MCP twojego klienta (`claude_desktop_config.json` dla Claude Desktop, `mcp.json` dla Cursor):

[inline-code-attrs-start title = 'Konfiguracja klienta MCP'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Bezpieczeństwo

Klucz API jest osadzony w URL. Traktuj URL jak sekret: nie wklejaj go do publicznych czatów, zrzutów ekranu ani commitów. Jeśli klucz zostanie ujawniony, zresetuj go na stronie Klucze API w panelu.