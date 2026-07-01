## Parameters

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| domainToUpdate | string | Yes |  |
| patchDomainConfigParams | PatchDomainConfigParams | Yes |  |

## Response

Враћа: [`PatchDomainConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PatchDomainConfigResponse.ts)

## Example

[inline-code-attrs-start title = 'patchDomainConfig Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function updateDomainConfig() {
  const tenantId: string = "tenant_98765";
  const domainToUpdate: string = "forum.mycompany.com";
  const patchParams: PatchDomainConfigParams = {
    enableComments: true,
    moderationLevel: "strict",
    allowAnonymous: false, // показан опционо параметар
  };
  const response: PatchDomainConfigResponse = await patchDomainConfig(tenantId, domainToUpdate, patchParams);
  console.log(response);
}

updateDomainConfig();
[inline-code-end]