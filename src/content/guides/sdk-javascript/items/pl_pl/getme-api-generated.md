Identyfikuje używane poświadczenie: dzierżawcę, do którego należy, a w przypadku tokenów OAuth — użytkownika, który je autoryzował.
Integracje używają tego do przetestowania połączenia i nadania mu etykiety.

## Parametry

| Nazwa | Typ | Wymagany | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |

## Odpowiedź

Zwraca: [`GetMeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetMeResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getMe'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-tenant";
  const me: GetMeResponse = await getMe(tenantId);
  const authType: MeAuthType = me.auth.type;
  const scopes: OAuthScope[] = me.auth.scopes ?? [];
  const status: APIStatus = me.status;
})();
[inline-code-end]