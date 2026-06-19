## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|--------------|
| tenantId | string | Ja |  |
| domainToUpdate | string | Ja |  |
| updateDomainConfigParams | UpdateDomainConfigParams | Ja |  |

## Respons

Retourneert: [`PutDomainConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PutDomainConfigResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'putDomainConfig Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "7f12c9a4-3b6e-4d2f-9a1c-5f8b2e0a91c4";
const domainToUpdate: string = "comments.newsroom-prod.com";
const updateParams: UpdateDomainConfigParams = {
  forceHttps: true,
  enableCORS: true,               // optionele vlag (demonstreert optionele parameters)
  corsAllowedOrigins: ["https://newsroom-prod.com"]
};
const response: PutDomainConfigResponse = await putDomainConfig(tenantId, domainToUpdate, updateParams);
console.log(response);
[inline-code-end]