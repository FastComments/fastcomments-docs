## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| commentId | string | Так |  |
| broadcastId | string | Ні |  |
| tenantId | string | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`PostRemoveCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostRemoveCommentResponse.ts)

## Приклад

[inline-code-attrs-start title = 'postRemoveComment Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function removeCommentExample(): Promise<void> {
  const commentId: string = "cmt_7f9a3e2b";

  const result1: PostRemoveCommentResponse = await postRemoveComment(commentId);

  const broadcastId: string = "brd_12ab34cd";
  const tenantId: string = "tenant_5678efgh";
  const sso: string = "sso_XYZ1235";

  const result2: PostRemoveCommentResponse = await postRemoveComment(commentId, broadcastId, tenantId, sso);

  console.log(result1, result2);
}
[inline-code-end]

---