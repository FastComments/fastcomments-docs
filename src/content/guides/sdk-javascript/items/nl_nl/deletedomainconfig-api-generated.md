## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| domain | string | Yes |  |

## Respons

Retourneert: [`DeleteDomainConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteDomainConfigResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'deleteDomainConfig Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = 'acme-corp';
  const domain: string = 'blog.acme.com';
  const response: DeleteDomainConfigResponse = await deleteDomainConfig(tenantId, domain);
  console.log(response);
}
runExample();
[inline-code-end]