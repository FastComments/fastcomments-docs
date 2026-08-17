## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|------|
| commentId | string | Да |  |
| broadcastId | string | Не |  |
| tenantId | string | Не |  |
| sso | string | Не |  |

## Одговор

Враћа: [`PostRemoveCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostRemoveCommentResponse.ts)

## Пример

[inline-code-attrs-start title = 'postRemoveComment Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function removeCommentExample(): Promise<void> {
  const commentId: string = "cmt_7f9a3e2b";

  const result1: PostRemoveCommentResponse = await postRemoveComment(commentId);

  const broadcastId: string = "brd_12ab34cd";
  const tenantId: string = "tenant_5678efgh";
  const sso: string = "sso_XYZ12345";

  const result2: PostRemoveCommentResponse = await postRemoveComment(commentId, broadcastId, tenantId, sso);

  console.log(result1, result2);
}
[inline-code-end]