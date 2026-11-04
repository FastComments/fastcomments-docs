댓글에서 투표를 제거하며, 해당 투표에 대한 모든 투표 기록도 함께 삭제합니다. 댓글 자체는 그대로 유지됩니다.

## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |

## 응답

반환값: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 예제

[inline-code-attrs-start title = 'deletePoll 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runDeletePoll(): Promise<void> {
  const tenantId: string = "tenant_9f8b7c6d";
  const commentId: string = "comment_a1b2c3d4";
  const result: APIEmptyResponse = await deletePoll(tenantId, commentId);
  console.log(result);
}
[inline-code-end]