## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| domain | string | Da |  |

## Odgovor

Vrne: [`DeleteDomainConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteDomainConfigResponse.ts)

## Primer

[inline-code-attrs-start title = 'deleteDomainConfig Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = 'acme-corp';
  const domain: string = 'blog.acme.com';
  const response: DeleteDomainConfigResponse = await deleteDomainConfig(tenantId, domain);
  console.log(response);
}
runExample();
[inline-code-end]

---