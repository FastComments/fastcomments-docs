## Параметри

| Name | Type | Обов'язкове | Опис |
|------|------|-------------|------|
| tenantId | string | Так |  |
| id | string | Так |  |
| editKey | string | Ні |  |

## Відповідь

Повертає: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteDeleteResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад deleteVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f7c2b1a';
const id: string = 'vote_4b6e9a23';
const editKey: string = 'editkey_02a8f3';

const deleteResultWithoutKey: VoteDeleteResponse = await deleteVote(tenantId, id);
const deleteResultWithKey: VoteDeleteResponse = await deleteVote(tenantId, id, editKey);
[inline-code-end]

---