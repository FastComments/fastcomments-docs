## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| commentId | string | Oui |  |
| tenantId | string | Non |  |
| sso | string | Non |  |

## Réponse

Renvoie : [`GetCommentBanStatusResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentBanStatusResponse1.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getCommentBanStatus'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function checkCommentBanStatus() {
  const banStatus: GetCommentBanStatusResponse1 = await getCommentBanStatus('cmt_987654321', 'tenant_42', 'sso_token_abc123');
  const banStatusNoTenant: GetCommentBanStatusResponse1 = await getCommentBanStatus('cmt_987654322', undefined, 'sso_token_def456');
}
[inline-code-end]