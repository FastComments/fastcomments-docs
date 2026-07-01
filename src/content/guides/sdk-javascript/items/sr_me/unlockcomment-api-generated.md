## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| commentId | string | Da |  |
| broadcastId | string | Da |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`UnLockCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnLockCommentResponse.ts)

## Primjer

[inline-code-attrs-start title = 'unLockComment Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "cmt_9876";
const broadcastId: string = "brd_001";
const ssoToken: string | undefined = "sso_token_abc";

async function run() {
  const unlocked: UnLockCommentResponse = await unLockComment(tenantId, commentId, broadcastId, ssoToken);
  console.log(unlocked);

  // Poziv bez opcionalnog sso parametra
  const unlockedWithoutSso: UnLockCommentResponse = await unLockComment(tenantId, commentId, broadcastId);
  console.log(unlockedWithoutSso);
}

run();
[inline-code-end]