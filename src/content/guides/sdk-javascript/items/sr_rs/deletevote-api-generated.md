## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| editKey | string | Не |  |

## Одговор

Враћа: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteCommentVote200Response.ts)

## Пример

[inline-code-attrs-start title = 'deleteVote пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f3b21c9';
const id: string = 'vote_4a2d9f1b';
const editKey: string = 'edit_92b7c6a1';

const resultWithoutEditKey: DeleteCommentVote200Response = await deleteVote(tenantId, id);
const resultWithEditKey: DeleteCommentVote200Response = await deleteVote(tenantId, id, editKey);
[inline-code-end]