FastComments uruchamia hostowany serwer Model Context Protocol (MCP), dzięki czemu asystenci AI i klienci agentowi mogą wywoływać API FastComments bezpośrednio. Każde narzędzie udostępniane przez serwer MCP jest automatycznie generowane na podstawie publicznej specyfikacji OpenAPI, więc wszystko, co może zrobić REST API, może zrobić również klient MCP.

Endpoint jest bezstanowy i oparty na strumieniowym HTTP. Nie ma sesji do utrzymania oraz nie istnieje stan po stronie serwera dla poszczególnych klientów.

### Endpoint

[inline-code-attrs-start title = 'Punkt końcowy MCP'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp
[inline-code-end]

### Connect with OAuth

Każdy klient MCP, który obsługuje zdalne serwery z OAuth (Claude, ChatGPT, Claude Code, Cursor i inne), może połączyć się z powyższym endpointem bez dodatkowej konfiguracji po stronie FastComments. Klient rejestruje się poprzez Dynamic Client Registration lub identyfikuje się za pomocą dokumentu metadanych Client ID, otwiera przeglądarkę, abyś mógł zalogować się do FastComments i zatwierdzić dostęp, a następnie otrzymuje token powiązany z kontem, na które jesteś zalogowany.

Dokumenty odkrywania znajdują się w standardowych lokalizacjach:

[inline-code-attrs-start title = 'Odkrywanie'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-protected-resource/mcp
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Twój użytkownik potrzebuje uprawnienia API Admin na koncie, aby zatwierdzić połączenie. Jeśli zarządzasz wieloma kontami, przełącz się na właściwe w panelu przed zatwierdzeniem.

Klient może żądać zakresu `read`, zakresu `write` lub obu. Klient, który nie żąda żadnego zakresu, otrzymuje oba. Narzędzia zmieniające dane nie są oferowane tokenowi tylko do odczytu.

Panel ma kreatora konfiguracji z gotowymi do wklejenia fragmentami kodu. Otwórz **Integrate -> MCP Server** lub przejdź bezpośrednio:

[inline-code-attrs-start title = 'Strona konfiguracji'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

### Claude Code

Zarejestruj serwer FastComments jednym poleceniem, a następnie uruchom `/mcp` w sesji, aby zalogować się i wyświetlić dostępne narzędzia:

[inline-code-attrs-start title = 'Konfiguracja Claude Code'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments https://fastcomments.com/mcp
[inline-code-end]

### Cursor and other config-file clients

Dodaj ten blok do konfiguracji serwerów MCP swojego klienta (`mcp.json` dla Cursor). Klient otworzy przeglądarkę, aby zalogować się przy pierwszym użyciu.

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

### Revoking access

Każde zatwierdzone połączenie jest wymienione w **Integrate -> Connected Apps** w panelu. Cofnięcie jednego połączenia unieważnia każdy token, który dana aplikacja posiada. Aplikacje rejestrują się samodzielnie przy połączeniu, a FastComments ich nie weryfikuje, więc odwołaj wszystko, czego nie rozpoznajesz.

### Using the token with the REST API

Token dostępu uzyskany przez klienta MCP jest zwykłym poświadczeniem API FastComments. Działa on na każdym endpointzie `/api/v1` jako token typu bearer, więc aplikacja połączona przez MCP może również wywoływać REST API bezpośrednio:

[inline-code-attrs-start title = 'Token Bearer'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -H "Authorization: Bearer fcat_..." https://fastcomments.com/api/v1/comments
[inline-code-end]

Tenant jest domyślnie określany na podstawie tokenu. `tenantId` może nadal być przekazywany, ale musi się zgadzać. Żądania `GET` wymagają zakresu `read`, a wszystkie pozostałe wymagają zakresu `write`.

### Connect with an API key

Klienci, którzy nie mogą przeprowadzić logowania w przeglądarce, np. serwery bez interfejsu graficznego, mogą uwierzytelnić się przy użyciu klucza API. Przekaż `tenantId` i `API_KEY` jako parametry zapytania lub jako nagłówki HTTP `x-tenant-id` i `x-api-key`, jeśli Twój klient obsługuje niestandardowe nagłówki:

[inline-code-attrs-start title = 'Endpoint klucza API'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

Strona konfiguracji generuje ten URL dla każdego z Twoich kluczy API.

### Security

URL endpointu zawierający klucz API jest tajny: nie wklejaj go w publicznych czatach, zrzutach ekranu ani w commitach. Jeśli klucz zostanie ujawniony, zresetuj go na stronie API Keys w panelu. Tokeny OAuth nie niosą takiego ryzyka, ponieważ są powiązane z jedną aplikacją i mogą być odwołane w sekcji Connected Apps.