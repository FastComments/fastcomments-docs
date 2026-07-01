req
tenantId
urlId
userIdWS

## パラメータ

| 名前 | タイプ | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| userIdWS | string | Yes |  |
| startTime | number | Yes |  |
| endTime | number | No |  |

## レスポンス

戻り値: [`GetGlobalEventLogResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetGlobalEventLogResponse.ts)

## 例

[inline-code-attrs-start title = 'getGlobalEventLog の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = '123e4567-e89b-12d3-a456-426614174000';
  const urlId: string = 'article-2023-09-15';
  const userIdWS: string = 'ws_987654321';
  const startTime: number = Date.now() - 86400000;
  const endTime: number = Date.now();

  const fullLog: GetGlobalEventLogResponse = await getGlobalEventLog(tenantId, urlId, userIdWS, startTime, endTime);
  const recentLog: GetGlobalEventLogResponse = await getGlobalEventLog(tenantId, urlId, userIdWS, startTime);

  console.log(fullLog, recentLog);
})();
[inline-code-end]

---