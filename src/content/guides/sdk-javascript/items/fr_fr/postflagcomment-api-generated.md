## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Réponse

Renvoie : [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple postFlagComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = "tenant_12345";
  const commentId: string = "cmt_98765";

  // Paramètres obligatoires uniquement
  const result1: APIEmptyResponse = await postFlagComment(tenantId, commentId);

  // Inclure les paramètres optionnels
  const broadcastId: string = "brd_54321";
  const sso: string = "user@example.com";
  const result2: APIEmptyResponse = await postFlagComment(tenantId, commentId, broadcastId, sso);

  console.log(result1, result2);
}
runExample();
[inline-code-end]