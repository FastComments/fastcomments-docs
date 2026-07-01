## 參數

| 名稱 | 類型 | 必要 | 描述 |
|------|------|------|------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| redirectURL | string | 否 |  |

## 回應

回傳：[`SendLoginLinkResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SendLoginLinkResponse.ts)

## 範例

[inline-code-attrs-start title = 'sendLoginLink 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoSendLoginLink() {
  const tenantId: string = "acme-corp";
  const userId: string = "user-9876";
  const redirectURL: string = "https://app.acme-corp.com/auth/callback";

  const resultWithRedirect: SendLoginLinkResponse = await sendLoginLink(tenantId, userId, redirectURL);
  const resultWithoutRedirect: SendLoginLinkResponse = await sendLoginLink(tenantId, userId);
}
[inline-code-end]