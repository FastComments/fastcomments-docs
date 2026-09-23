## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| broadcastId | string | Yes |  |
| sso | string | No |  |

## 応答

返却: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 例

[inline-code-attrs-start title = 'unLockComment の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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