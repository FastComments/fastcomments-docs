## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| commentId | string | はい |  |
| broadcastId | string | はい |  |
| editKey | string | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`DeleteCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteCommentPublic200Response.ts)

## 例

[inline-code-attrs-start title = 'deleteCommentPublic の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_42';
const commentId: string = 'c0mment-9f8b7a6';
const broadcastId: string = 'site_homepage_2026-06-15';
const editKey: string = 'ek_3b7a1f59-4d2c-11eb-8dcd-0242ac130003';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.fakePayload.signature';

const result: DeleteCommentPublic200Response = await deleteCommentPublic(tenantId, commentId, broadcastId, editKey, sso);
[inline-code-end]

---