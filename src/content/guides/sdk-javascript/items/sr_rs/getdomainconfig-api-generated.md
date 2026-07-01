## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| domain | string | Da |  |

## Odgovor

Vraća: [`GetDomainConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetDomainConfigResponse.ts)

## Primer

[inline-code-attrs-start title = 'getDomainConfig Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function main() {
  const tenantId: string = "acme-corp-123";
  const domain: string = "blog.acme.com";
  const config: GetDomainConfigResponse = await getDomainConfig(tenantId, domain);
  console.log(config);
}
main();
[inline-code-end]