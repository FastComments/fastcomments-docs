## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| commentId | string | Да |  |
| editKey | string | Не |  |
| sso | string | Не |  |

## Одговор

Враћа: [`GetCommentText200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentText200Response.ts)

## Пример

[inline-code-attrs-start title = 'getCommentText Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_001';
const commentId: string = 'cmt_7890b';
const editKey: string = 'edit_4f2d9b7c';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9';

const result: GetCommentText200Response = await getCommentText(tenantId, commentId, editKey, sso);
[inline-code-end]

---