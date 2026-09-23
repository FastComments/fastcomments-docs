---
## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| broadcastId | string | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 예시

[inline-code-attrs-start title = 'postFlagComment 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = "tenant_12345";
  const commentId: string = "cmt_98765";

  // 필수 매개변수만
  const result1: APIEmptyResponse = await postFlagComment(tenantId, commentId);

  // 선택적 매개변수 포함
  const broadcastId: string = "brd_54321";
  const sso: string = "user@example.com";
  const result2: APIEmptyResponse = await postFlagComment(tenantId, commentId, broadcastId, sso);

  console.log(result1, result2);
}
runExample();
[inline-code-end]

---