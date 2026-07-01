## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| commentId | string | Oui |  |
| broadcastId | string | Oui |  |
| sso | string | Non |  |

## Réponse

Renvoie : [`UnLockCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnLockCommentResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple unLockComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "cmt_9876";
const broadcastId: string = "brd_001";
const ssoToken: string | undefined = "sso_token_abc";

async function run() {
  const unlocked: UnLockCommentResponse = await unLockComment(tenantId, commentId, broadcastId, ssoToken);
  console.log(unlocked);

  // Appel sans le paramètre sso optionnel
  const unlockedWithoutSso: UnLockCommentResponse = await unLockComment(tenantId, commentId, broadcastId);
  console.log(unlockedWithoutSso);
}

run();
[inline-code-end]

---