## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |

## Réponse

Renvoie : [`GetDomainConfigsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetDomainConfigsResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getDomainConfigs'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchDomainConfigs(): Promise<void> {
  const tenantId: string = "acme-corp-567";
  const configs: GetDomainConfigsResponse = await getDomainConfigs(tenantId);
  console.log(configs);
}
[inline-code-end]