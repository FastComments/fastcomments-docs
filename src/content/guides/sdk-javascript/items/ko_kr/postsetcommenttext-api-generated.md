## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| setCommentTextParams | SetCommentTextParams | 예 |  |
| broadcastId | string | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`SetCommentTextResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SetCommentTextResponse.ts)

## 예시

[inline-code-attrs-start title = 'postSetCommentText 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---