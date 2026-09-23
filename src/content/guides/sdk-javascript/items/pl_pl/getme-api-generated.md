Identifies the credential in use: the tenant it belongs to and, for OAuth tokens, the user who authorized it.  
Integrations use this to test a connection and label it.

## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
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