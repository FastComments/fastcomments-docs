## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| postIds | Array<string> | Nie |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`GetUserReactsPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserReactsPublic200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getUserReactsPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-tenant-8a4d2c";
const postIds: string[] = ["post-645a2f", "post-645a30"];
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyXzEyMyIsImlhdCI6MTY2MTYwMDAwMH0.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
const result: GetUserReactsPublic200Response = await getUserReactsPublic(tenantId, postIds, sso);
[inline-code-end]