## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| domainToUpdate | string | Да |  |
| updateDomainConfigParams | UpdateDomainConfigParams | Да |  |

## Ответ

Возвращает: [`PutDomainConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PutDomainConfigResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример putDomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "7f12c9a4-3b6e-4d2f-9a1c-5f8b2e0a91c4";
const domainToUpdate: string = "comments.newsroom-prod.com";
const updateParams: UpdateDomainConfigParams = {
  forceHttps: true,
  enableCORS: true,               // необязательный флаг (демонстрирует необязательные параметры)
  corsAllowedOrigins: ["https://newsroom-prod.com"]
};
const response: PutDomainConfigResponse = await putDomainConfig(tenantId, domainToUpdate, updateParams);
console.log(response);
[inline-code-end]