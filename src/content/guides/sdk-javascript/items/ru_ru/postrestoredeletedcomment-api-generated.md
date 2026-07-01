## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | Да |  |
| broadcastId | string | Нет |  |
| tenantId | string | Нет |  |
| sso | string | Нет |  |

## Ответ

Возвращает: [`PostRestoreDeletedCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostRestoreDeletedCommentResponse.ts)

## Пример

[inline-code-attrs-start title = 'postRestoreDeletedComment Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function restoreCommentDemo(): Promise<void> {
    const commentId: string = "cmt_5f2a9b7e1234567890abcd";
    const broadcastId: string | undefined = "brd_2023_09";
    const tenantId: string | undefined = "tenant_42";
    const sso: string | undefined = "sso_token_abcdef123456";

    const response: PostRestoreDeletedCommentResponse = await postRestoreDeletedComment(
        commentId,
        broadcastId,
        tenantId,
        sso
    );

    console.log(response);
}

restoreCommentDemo();
[inline-code-end]