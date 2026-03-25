## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 回應

回傳：[`GetComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetComment200Response.ts)

## 範例

[inline-code-attrs-start title = 'getComment 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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