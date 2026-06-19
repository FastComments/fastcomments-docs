## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| dir | number | Yes |  |
| sso | string | No |  |

## Odgovor

Vraća: [`GetCommentVoteUserNamesSuccessResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentVoteUserNamesSuccessResponse.ts)

## Primjer

[inline-code-attrs-start title = 'getCommentVoteUserNames Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-7f9c';
const commentId: string = 'bcd12345-6789-4ef0-9abc-0d1e2f3a4b5c';
const dir: number = 1;
const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.ssoPayload.signature';

const resultWithoutSso: GetCommentVoteUserNamesSuccessResponse = await getCommentVoteUserNames(tenantId, commentId, dir);
const resultWithSso: GetCommentVoteUserNamesSuccessResponse = await getCommentVoteUserNames(tenantId, commentId, dir, ssoToken);
[inline-code-end]