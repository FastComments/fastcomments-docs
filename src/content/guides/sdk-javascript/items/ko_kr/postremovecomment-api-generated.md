## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|------|------|
| commentId | string | 예 |  |
| broadcastId | string | 아니요 |  |
| tenantId | string | 아니요 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`PostRemoveCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostRemoveCommentResponse.ts)

## 예제

[inline-code-attrs-start title = 'postRemoveComment 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function removeCommentExample(): Promise<void> {
  const commentId: string = "cmt_7f9a3e2b";

  const result1: PostRemoveCommentResponse = await postRemoveComment(commentId);

  const broadcastId: string = "brd_12ab34cd";
  const tenantId: string = "tenant_5678efgh";
  const sso: string = "sso_XYZ12345";

  const result2: PostRemoveCommentResponse = await postRemoveComment(commentId, broadcastId, tenantId, sso);

  console.log(result1, result2);
}
[inline-code-end]