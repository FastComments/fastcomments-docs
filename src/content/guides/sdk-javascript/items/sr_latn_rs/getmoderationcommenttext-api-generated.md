## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| sso | string | No |  |

## Odgovor

Vraća: [`GetCommentTextResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentTextResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer getModerationCommentText'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchComment() {
  const tenantId: string = "tenant_12345";
  const commentId: string = "cmt_98765";
  const ssoToken: string = "sso_abcde12345";

  const commentWithoutSso: GetCommentTextResponse = await getModerationCommentText(tenantId, commentId);
  const commentWithSso: GetCommentTextResponse = await getModerationCommentText(tenantId, commentId, ssoToken);
}

fetchComment();
[inline-code-end]