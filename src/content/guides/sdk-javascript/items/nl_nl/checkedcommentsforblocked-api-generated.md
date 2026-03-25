## Parameters

| Name | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentIds | string | Ja |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`CheckedCommentsForBlocked200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CheckedCommentsForBlocked200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'checkedCommentsForBlocked Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_4f3b2a1e';
const commentIds: string = 'c_1001,c_1002,c_1003';
const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIn0.Sf4ke7nQP3mZx9v2';

const resultWithoutSSO: CheckedCommentsForBlocked200Response = await checkedCommentsForBlocked(tenantId, commentIds);
const resultWithSSO: CheckedCommentsForBlocked200Response = await checkedCommentsForBlocked(tenantId, commentIds, ssoToken);
[inline-code-end]

---