## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| redirectURL | string | No |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 예제

[inline-code-attrs-start title = 'sendLoginLink 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface APIStatus {
  // 상태 필드
}

interface APIEmptyResponse {
  status: APIStatus;
}

(async () => {
  const tenantId: string = "acme-corp";
  const userId: string = "user-98765";
  const redirectURL: string = "https://app.example.com/welcome";

  const responseWithRedirect: APIEmptyResponse = await sendLoginLink(tenantId, userId, redirectURL);
  const responseWithoutRedirect: APIEmptyResponse = await sendLoginLink(tenantId, userId);
})();
[inline-code-end]