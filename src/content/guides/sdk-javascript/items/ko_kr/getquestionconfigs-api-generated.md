## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| skip | number | 아니요 |  |

## 응답

반환: [`GetQuestionConfigs200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfigs200Response.ts)

## 예제

[inline-code-attrs-start title = 'getQuestionConfigs 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acme_9876";
const configsWithoutSkip: GetQuestionConfigs200Response = await getQuestionConfigs(tenantId);
const configsWithSkip: GetQuestionConfigs200Response = await getQuestionConfigs(tenantId, 20);
[inline-code-end]

---