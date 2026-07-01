## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| commentId | string | Tak |  |
| broadcastId | string | Tak |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`UnLockCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnLockCommentResponse.ts)

## Przykład

[inline-code-attrs-start title = 'unLockComment Przykład'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "cmt_9876";
const broadcastId: string = "brd_001";
const ssoToken: string | undefined = "sso_token_abc";

async function run() {
  const unlocked: UnLockCommentResponse = await unLockComment(tenantId, commentId, broadcastId, ssoToken);
  console.log(unlocked);

  // Wywołaj bez opcjonalnego parametru sso
  const unlockedWithoutSso: UnLockCommentResponse = await unLockComment(tenantId, commentId, broadcastId);
  console.log(unlockedWithoutSso);
}

run();
[inline-code-end]