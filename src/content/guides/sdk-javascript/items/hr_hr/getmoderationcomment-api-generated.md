## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| commentId | string | Da |  |
| includeEmail | boolean | Ne |  |
| includeIP | boolean | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPICommentResponse.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer getModerationComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchComments() {
  const tenantId: string = "acme-corp";
  const commentId: string = "cmt_1234567890";

  // Poziv s samo obaveznim parametrima
  const basicResponse: ModerationAPICommentResponse = await getModerationComment(tenantId, commentId);

  // Poziv s opcionalnim parametrima
  const includeEmail: boolean = true;
  const includeIP: boolean = false;
  const sso: string = "sso-token-abc123";
  const detailedResponse: ModerationAPICommentResponse = await getModerationComment(
    tenantId,
    commentId,
    includeEmail,
    includeIP,
    sso
  );
}
[inline-code-end]

---