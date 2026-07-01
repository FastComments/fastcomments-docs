## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| commentId | string | Oui |  |
| editKey | string | Non |  |
| sso | string | Non |  |

## Réponse

Retourne : [`GetCommentTextResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentTextResponse1.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getCommentText'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function run() {
  const tenantId: string = "acme-corp-tenant";
  const commentId: string = "cmt-5f2e9a1b";
  const editKey: string = "edk-9b7c3";
  const ssoToken: string = "sso-xyz789";

  const commentOnly: GetCommentTextResponse1 = await getCommentText(tenantId, commentId);
  const commentWithEdit: GetCommentTextResponse1 = await getCommentText(tenantId, commentId, editKey);
  const commentFull: GetCommentTextResponse1 = await getCommentText(tenantId, commentId, editKey, ssoToken);
}
run();
[inline-code-end]