## Parameters

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| commentId | string | Yes |  |
| includeEmail | boolean | No |  |
| includeIP | boolean | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## Odgovor

Vraća: [`GetModerationCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerationCommentResponse.ts)

## Primjer

[inline-code-attrs-start title = 'getModerationComment Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchCommentDetails() {
  // Cijeli set parametara
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

  // Minimalni poziv koristeći samo obavezni argument
  const minimalResult: GetModerationCommentResponse = await getModerationComment("cmt_67890def");

  // Koristite rezultate po potrebi...
}
[inline-code-end]