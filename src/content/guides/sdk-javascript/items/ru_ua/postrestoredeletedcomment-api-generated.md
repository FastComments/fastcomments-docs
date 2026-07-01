## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|--------------|------|
| commentId | string | Yes |  |
| broadcastId | string | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## Відповідь

Повертає: [`PostRestoreDeletedCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostRestoreDeletedCommentResponse.ts)

## Приклад

[inline-code-attrs-start title = 'postRestoreDeletedComment Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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