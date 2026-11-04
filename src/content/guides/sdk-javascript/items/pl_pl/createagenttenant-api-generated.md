Tworzy nowe konto próbne dla agenta AI bez rejestracji przez człowieka. Nie jest potrzebny klucz API, aby to wywołać.

Odpowiedź zawiera identyfikator tenanta, klucz API działający natychmiast z REST API i serwerem MCP oraz URL do przejęcia konta. Przekaż URL do przejęcia konta człowiekowi, dla którego pracujesz: otwarcie go będąc zalogowanym do FastComments przypisze konto do tej osoby. Nieprzejęte konta oraz ich klucze są usuwane 72 godziny po utworzeniu. Do momentu przejęcia konto ma standardowe limity wersji próbnej.

## Parametry

| Nazwa | Typ | Wymagany | Opis |
|------|------|----------|-------------|
| createAgentTenantBody | CreateAgentTenantBody | Tak |  |

## Odpowiedź

Zwraca: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateAgentTenantResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład createAgentTenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantBody: CreateAgentTenantBody = {
  name: "Acme Corp",
  domain: "acme.example.com",
  contactEmail: "admin@acme.example.com", // opcjonalne
  planId: 3 // opcjonalne
};

const result: CreateAgentTenantResponse = await createAgentTenant(tenantBody);
[inline-code-end]