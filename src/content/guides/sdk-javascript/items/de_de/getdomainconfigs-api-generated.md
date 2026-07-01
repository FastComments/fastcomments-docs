## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |

## Antwort

Returns: [`GetDomainConfigsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetDomainConfigsResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für getDomainConfigs'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchDomainConfigs(): Promise<void> {
  const tenantId: string = "acme-corp-567";
  const configs: GetDomainConfigsResponse = await getDomainConfigs(tenantId);
  console.log(configs);
}
[inline-code-end]