## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| skip | number | 否 |  |

## 回應

回傳： [`GetQuestionConfigs200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfigs200Response.ts)

## 範例

[inline-code-attrs-start title = 'getQuestionConfigs 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_4c9f2b";
const responseWithoutSkip: GetQuestionConfigs200Response = await getQuestionConfigs(tenantId);
const skip: number = 50;
const responseWithSkip: GetQuestionConfigs200Response = await getQuestionConfigs(tenantId, skip);
[inline-code-end]