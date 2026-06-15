## Parametre

| Name | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| dir | number | Ja |  |
| sso | string | Nej |  |

## Svar

Returnerer: [`GetCommentVoteUserNames200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentVoteUserNames200Response.ts)

## Eksempel

[inline-code-attrs-start title = 'getCommentVoteUserNames Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_67890";
const commentId: string = "comment_abc123";
const dir: number = 1;
const ssoToken: string = "sso-eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9";

const responseWithoutSSO: GetCommentVoteUserNames200Response = await getCommentVoteUserNames(tenantId, commentId, dir);
const responseWithSSO: GetCommentVoteUserNames200Response = await getCommentVoteUserNames(tenantId, commentId, dir, ssoToken);
[inline-code-end]