## Parametry

| Name | Type | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| id | string | Tak |  |
| editKey | string | Nie |  |

## Odpowiedź

Zwraca: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteDeleteResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład deleteVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f7c2b1a';
const id: string = 'vote_4b6e9a23';
const editKey: string = 'editkey_02a8f3';

const deleteResultWithoutKey: VoteDeleteResponse = await deleteVote(tenantId, id);
const deleteResultWithKey: VoteDeleteResponse = await deleteVote(tenantId, id, editKey);
[inline-code-end]

---