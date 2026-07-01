## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| urlId | string | はい |  |
| tenantId | string | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

返却: [`PutReopenThreadResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PutReopenThreadResponse.ts)

## 例

[inline-code-attrs-start title = 'putReopenThread の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function exampleUsage() {
  const urlId: string = "thread-9f8b7c6a";
  const tenantId: string = "tenant-001";
  const sso: string = "sso-3f9d2e1a";

  const resultAll: PutReopenThreadResponse = await putReopenThread(urlId, tenantId, sso);
  console.log(resultAll);

  const resultMinimal: PutReopenThreadResponse = await putReopenThread(urlId);
  console.log(resultMinimal);
}
[inline-code-end]

---