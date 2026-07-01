## パラメータ

| 名前 | 種類 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| redirectURL | string | いいえ |  |

## レスポンス

返却: [`SendLoginLinkResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SendLoginLinkResponse.ts)

## 例

[inline-code-attrs-start title = 'sendLoginLink の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoSendLoginLink() {
  const tenantId: string = "acme-corp";
  const userId: string = "user-9876";
  const redirectURL: string = "https://app.acme-corp.com/auth/callback";

  const resultWithRedirect: SendLoginLinkResponse = await sendLoginLink(tenantId, userId, redirectURL);
  const resultWithoutRedirect: SendLoginLinkResponse = await sendLoginLink(tenantId, userId);
}
[inline-code-end]