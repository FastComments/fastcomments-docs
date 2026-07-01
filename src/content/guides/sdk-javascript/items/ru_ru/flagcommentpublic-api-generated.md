## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| commentId | string | Да |  |
| isFlagged | boolean | Да |  |
| sso | string | Нет |  |

## Ответ

Возвращает: [`FlagCommentPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublicResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример flagCommentPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoFlagComment() {
    const tenantId: string = "acme-tenant-123";
    const commentId: string = "comment-987654";
    const isFlagged: boolean = true;
    const sso: string = "sso-token-abc123";

    const result: FlagCommentPublicResponse = await flagCommentPublic(
        tenantId,
        commentId,
        isFlagged,
        sso
    );

    console.log(result);
}

demoFlagComment();
[inline-code-end]