## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| editKey | string | Нет |  |

## Ответ

Возвращает: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteCommentVote200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример deleteVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '123e4567-e89b-12d3-a456-426614174000';
const id: string = 'vote-7a1b2c3d-9f8e-4b6a-8123-abcdef012345';
const editKey: string = 'editKey_4f3e2d1c';

const resultWithEditKey: DeleteCommentVote200Response = await deleteVote(tenantId, id, editKey);
const resultWithoutEditKey: DeleteCommentVote200Response = await deleteVote(tenantId, id);
[inline-code-end]

---