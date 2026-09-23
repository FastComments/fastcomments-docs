## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| postIds | Array<string> | Nie |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`UserReactsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UserReactsResponse.ts)

## Przykład

[inline-code-attrs-start title = 'getUserReactsPublic Przykład'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const postIds: string[] = ["post_abc", "post_def"];
const ssoToken: string = "sso_token_987";

const fullResponse: UserReactsResponse = await getUserReactsPublic(tenantId, postIds, ssoToken);
const minimalResponse: UserReactsResponse = await getUserReactsPublic(tenantId);
[inline-code-end]

---