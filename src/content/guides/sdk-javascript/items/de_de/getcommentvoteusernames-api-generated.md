## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| dir | number | Ja |  |
| sso | string | Nein |  |

## Antwort

Rückgabe: [`GetCommentVoteUserNamesSuccessResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentVoteUserNamesSuccessResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getCommentVoteUserNames Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function example() {
  const tenantId: string = 'tenant-9876';
  const commentId: string = 'comment-abc123';
  const dir: number = 1;
  const ssoToken: string = 'sso-xyz789';

  const resultWithSso: GetCommentVoteUserNamesSuccessResponse = await getCommentVoteUserNames(tenantId, commentId, dir, ssoToken);
  const resultWithoutSso: GetCommentVoteUserNamesSuccessResponse = await getCommentVoteUserNames(tenantId, commentId, dir);
}
[inline-code-end]