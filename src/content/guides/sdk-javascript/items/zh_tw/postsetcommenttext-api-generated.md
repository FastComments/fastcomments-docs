## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| setCommentTextParams | SetCommentTextParams | 是 |  |
| broadcastId | string | 否 |  |
| sso | string | 否 |  |

## 回應

返回：[`SetCommentTextResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SetCommentTextResponse.ts)

## 範例

[inline-code-attrs-start title = 'postSetCommentText 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "cmt_9876";
const setCommentTextParams: SetCommentTextParams = {
  text: "Revised comment after user edit.",
  format: "plain"
};
const broadcastId: string = "broadcast_2023_09";

const response: SetCommentTextResponse = await postSetCommentText(
  tenantId,
  commentId,
  setCommentTextParams,
  broadcastId
);

console.log(response);
[inline-code-end]