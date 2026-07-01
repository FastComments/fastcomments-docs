## Параметри

| Ім'я | Тип | Обов'язково | Опис |
|------|------|-------------|------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| broadcastId | string | Yes |  |
| sso | string | No |  |

## Відповідь

Повертає: [`UnLockCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnLockCommentResponse.ts)

## Приклад

[inline-code-attrs-start title = 'unLockComment Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "cmt_9876";
const broadcastId: string = "brd_001";
const ssoToken: string | undefined = "sso_token_abc";

async function run() {
  const unlocked: UnLockCommentResponse = await unLockComment(tenantId, commentId, broadcastId, ssoToken);
  console.log(unlocked);

  // Виклик без необов'язкового параметра sso
  const unlockedWithoutSso: UnLockCommentResponse = await unLockComment(tenantId, commentId, broadcastId);
  console.log(unlockedWithoutSso);
}

run();
[inline-code-end]