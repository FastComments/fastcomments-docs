## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| skip | number | いいえ |  |

## レスポンス

戻り値: [`GetQuestionConfigsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfigsResponse.ts)

## 例

[inline-code-attrs-start title = 'getQuestionConfigs の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async (): Promise<void> => {
  const tenantId: string = 'acme-corp-78';
  const responseWithoutSkip: GetQuestionConfigsResponse = await getQuestionConfigs(tenantId);
  const responseWithSkip: GetQuestionConfigsResponse = await getQuestionConfigs(tenantId, 25);
  console.log(responseWithoutSkip, responseWithSkip);
})();
[inline-code-end]

---