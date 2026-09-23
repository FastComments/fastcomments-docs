## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| broadcastId | string | 예 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 예제

[inline-code-attrs-start title = 'unLockComment 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoUnlock() {
  const tenantId: string = "acme-corp";
  const commentId: string = "cmt_1234567890";
  const broadcastId: string = "brd_987654321";
  const ssoToken: string = "sso_abcdef123456";

  const resultWithoutSso: APIEmptyResponse = await unLockComment(tenantId, commentId, broadcastId);
  const resultWithSso: APIEmptyResponse = await unLockComment(tenantId, commentId, broadcastId, ssoToken);
}
[inline-code-end]

---