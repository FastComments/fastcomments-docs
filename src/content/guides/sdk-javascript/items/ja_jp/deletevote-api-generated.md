## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| editKey | string | いいえ |  |

## レスポンス

返却値: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteCommentVote200Response.ts)

## 例

[inline-code-attrs-start title = 'deleteVote の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '123e4567-e89b-12d3-a456-426614174000';
const id: string = 'vote-7a1b2c3d-9f8e-4b6a-8123-abcdef012345';
const editKey: string = 'editKey_4f3e2d1c';

const resultWithEditKey: DeleteCommentVote200Response = await deleteVote(tenantId, id, editKey);
const resultWithoutEditKey: DeleteCommentVote200Response = await deleteVote(tenantId, id);
[inline-code-end]

---