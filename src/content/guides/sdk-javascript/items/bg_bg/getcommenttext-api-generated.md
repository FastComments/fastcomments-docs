## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| commentId | string | Да |  |
| editKey | string | Не |  |
| sso | string | Не |  |

## Отговор

Връща: [`PublicAPIGetCommentTextResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PublicAPIGetCommentTextResponse.ts)

## Пример

[inline-code-attrs-start title = 'getCommentText Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_prod_01';
const commentId: string = 'cmt_5f2d9b8a-3e7c-4a1b';
const editKey: string = 'edit_8b3f6c2d4a9e';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.example.signature';
const response: PublicAPIGetCommentTextResponse = await getCommentText(tenantId, commentId, editKey, sso);
[inline-code-end]

---