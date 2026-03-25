req
tenantId
urlId
userIdWS

## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| urlId | string | はい |  |
| userIdWS | string | はい |  |
| startTime | number | はい |  |
| endTime | number | はい |  |

## レスポンス

戻り値: [`GetEventLog200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEventLog200Response.ts)

## 例

[inline-code-attrs-start title = 'getGlobalEventLog の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant-84b2f1";
const urlId: string = "article-6721";
const userIdWS: string = "ws-conn-9a3c";
const startTime: number = Date.now() - 7 * 24 * 60 * 60 * 1000; // 7日前
const endTimeOptional: number | undefined = undefined; // 終了時刻（オプション）
const endTime: number = endTimeOptional ?? Date.now();
const eventLog: GetEventLog200Response = await getGlobalEventLog(tenantId, urlId, userIdWS, startTime, endTime);
[inline-code-end]

---