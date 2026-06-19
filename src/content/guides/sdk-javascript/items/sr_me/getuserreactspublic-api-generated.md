## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| postIds | Array<string> | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`UserReactsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UserReactsResponse.ts)

## Primjer

[inline-code-attrs-start title = 'getUserReactsPublic Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f3b2a9c';
const postIds: string[] = ['post_1a2b3c', 'post_4d5e6f'];
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTYifQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c';
const fullResponse: UserReactsResponse = await getUserReactsPublic(tenantId, postIds, sso);
const minimalResponse: UserReactsResponse = await getUserReactsPublic(tenantId)
[inline-code-end]

---