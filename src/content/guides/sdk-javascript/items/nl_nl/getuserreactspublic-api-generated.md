## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| postIds | Array<string> | No |  |
| sso | string | No |  |

## Respons

Retourneert: [`UserReactsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UserReactsResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getUserReactsPublic Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const postIds: string[] = ["post_abc", "post_def"];
const ssoToken: string = "sso_token_987";

const fullResponse: UserReactsResponse = await getUserReactsPublic(tenantId, postIds, ssoToken);
const minimalResponse: UserReactsResponse = await getUserReactsPublic(tenantId);
[inline-code-end]