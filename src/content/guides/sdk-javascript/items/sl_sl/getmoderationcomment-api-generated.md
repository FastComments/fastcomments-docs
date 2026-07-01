## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|------|
| commentId | string | Yes |  |
| includeEmail | boolean | No |  |
| includeIP | boolean | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## Odgovor

Vrne: [`GetModerationCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerationCommentResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer getModerationComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchCommentDetails() {
  // Polni nabor parametrov
  const commentId: string = "cmt_12345abc";
  const includeEmail: boolean = true;
  const includeIP: boolean = false;
  const tenantId: string = "tenant_9876";
  const sso: string = "sso_token_xyz";

  const fullResult: GetModerationCommentResponse = await getModerationComment(
    commentId,
    includeEmail,
    includeIP,
    tenantId,
    sso
  );

  // Minimalni klic z uporabo samo zahtevanega argumenta
  const minimalResult: GetModerationCommentResponse = await getModerationComment("cmt_67890def");

  // Uporabite rezultate po potrebi...
}
[inline-code-end]

---