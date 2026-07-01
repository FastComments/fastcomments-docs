## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| addDomainConfigParams | AddDomainConfigParams | Ja |  |

## Respons

Retourneert: [`AddDomainConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddDomainConfigResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'addDomainConfig Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_12345';
  const config: AddDomainConfigParams = {
    domain: 'myblog.example.com',
    enabled: true,
    // beschrijving is optioneel en hier weggelaten
  };
  const response: AddDomainConfigResponse = await addDomainConfig(tenantId, config);
  console.log(response);
})();
[inline-code-end]

---