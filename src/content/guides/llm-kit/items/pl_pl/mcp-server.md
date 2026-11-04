FastComments uruchamia hostowany serwer Model Context Protocol (MCP), dzięki czemu asystenci AI i klienci agentowi mogą bezpośrednio wywoływać API FastComments. Każde narzędzie udostępniane przez serwer MCP jest automatycznie generowane na podstawie publicznej specyfikacji OpenAPI, więc wszystko, co potrafi REST API, może zrobić klient MCP.

Endpoint jest bezstanowy i oparty na streamable-HTTP. Nie ma sesji do utrzymywania ani stanu po stronie serwera dla poszczególnych klientów.

### Endpoint

[inline-code-attrs-start title = 'Endpoint MCP'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp
[inline-code-end]

### Połączenie przez OAuth

Każdy klient MCP obsługujący zdalne serwery z OAuth (Claude, ChatGPT, Claude Code, Cursor i inne) może połączyć się z powyższym endpointem bez żadnej konfiguracji po stronie FastComments. Klient rejestruje się samodzielnie poprzez Dynamic Client Registration lub identyfikuje się za pomocą Client ID Metadata Document, otwiera przeglądarkę, abyś mógł zalogować się do FastComments i zatwierdzić dostęp, a następnie otrzymuje token powiązany z kontem, na które byłeś zalogowany.

Dokumenty discovery znajdują się w standardowych lokalizacjach:

[inline-code-attrs-start title = 'Discovery'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-protected-resource/mcp
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Twój użytkownik musi mieć uprawnienie API Admin na koncie, aby zatwierdzić połączenie. Jeśli zarządzasz kilkoma kontami, przełącz się na właściwe w panelu przed zatwierdzeniem.

Klient może zażądać scope `read`, scope `write` lub obu. Klient, który niczego nie zażąda, otrzymuje oba. Narzędzia zmieniające dane nie są udostępniane tokenowi tylko do odczytu.

Panel zawiera kreator konfiguracji z gotowymi do wklejenia fragmentami kodu. Otwórz **Integrate -> MCP Server** lub odwiedź go bezpośrednio:

[inline-code-attrs-start title = 'Strona konfiguracji'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

### Claude Code

Zarejestruj serwer FastComments jedną komendą, a następnie uruchom `/mcp` wewnątrz sesji, aby się zalogować i wyświetlić dostępne narzędzia:

[inline-code-attrs-start title = 'Konfiguracja Claude Code'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments https://fastcomments.com/mcp
[inline-code-end]

### Cursor i inni klienci z plikiem konfiguracyjnym

Dodaj ten blok do konfiguracji serwerów MCP swojego klienta (`mcp.json` w przypadku Cursor). Klient otworzy przeglądarkę do logowania przy pierwszym użyciu.

[inline-code-attrs-start title = 'Konfiguracja klienta MCP'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Odwoływanie dostępu

Każde zatwierdzone połączenie jest widoczne w sekcji **Integrate -> Connected Apps** w panelu. Odwołanie połączenia unieważnia wszystkie tokeny, które dana aplikacja posiada. Aplikacje rejestrują się same podczas łączenia, a FastComments ich nie weryfikuje, więc odwołaj wszystko, czego nie rozpoznajesz.

### Używanie tokenu z REST API

Token dostępu uzyskany przez klienta MCP to zwykłe poświadczenie API FastComments. Działa na każdym endpoincie `/api/v1` jako token bearer, więc aplikacja, która połączyła się przez MCP, może również wywoływać REST API bezpośrednio:

[inline-code-attrs-start title = 'Token Bearer'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -H "Authorization: Bearer fcat_..." https://fastcomments.com/api/v1/comments
[inline-code-end]

Tenant wynika z tokenu. `tenantId` nadal może być przekazany, ale musi się zgadzać. Żądania `GET` wymagają scope `read`, a wszystko inne wymaga scope `write`.

### Połączenie za pomocą klucza API

Klienci, którzy nie mogą ukończyć logowania przez przeglądarkę, na przykład serwery headless, mogą uwierzytelnić się za pomocą klucza API. Przekaż `tenantId` i `API_KEY` jako parametry zapytania lub jako nagłówki HTTP `x-tenant-id` i `x-api-key`, jeśli Twój klient obsługuje niestandardowe nagłówki:

[inline-code-attrs-start title = 'Endpoint z kluczem API'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

Strona konfiguracji generuje ten URL dla każdego z Twoich kluczy API.

### Bezpieczeństwo

URL endpointu zawierający klucz API jest tajemnicą: nie wklejaj go do publicznych czatów, zrzutów ekranu ani commitów. Jeśli klucz został ujawniony, zmień go na stronie API Keys w panelu. Tokeny OAuth nie niosą takiego ryzyka, ponieważ są powiązane z jedną aplikacją i można je odwołać w Connected Apps.