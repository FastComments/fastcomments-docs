## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| domainToUpdate | string | Da |  |
| updateDomainConfigParams | UpdateDomainConfigParams | Da |  |

## Odgovor

Vraća: [`PutDomainConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PutDomainConfigResponse.ts)

## Primjer

[inline-code-attrs-start title = 'putDomainConfig Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "7f12c9a4-3b6e-4d2f-9a1c-5f8b2e0a91c4";
const domainToUpdate: string = "comments.newsroom-prod.com";
const updateParams: UpdateDomainConfigParams = {
  forceHttps: true,
  enableCORS: true,               // neobavezna zastavica (pokazuje neobavezne parametre)
  corsAllowedOrigins: ["https://newsroom-prod.com"]
};
const response: PutDomainConfigResponse = await putDomainConfig(tenantId, domainToUpdate, updateParams);
console.log(response);
[inline-code-end]

---