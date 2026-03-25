## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |

## レスポンス

戻り値: [`GetComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetComment200Response.ts)

## 例

[inline-code-attrs-start title = 'getComment の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-publishing-001";
const commentId: string = "f3b2c1d0-9a8e-4b7c-8123-6d5f0a1e2b3c";
const result: GetComment200Response = await getComment(tenantId, commentId);
const wrapper: GetComment200Response & { comment?: APIComment } = result;
const comment: APIComment | undefined = wrapper.comment;
const authorBadge: CommentUserBadgeInfo | undefined = comment?.user?.badge;
const userHashTags: CommentUserHashTagInfo[] | undefined = comment?.user?.hashTags
[inline-code-end]

---