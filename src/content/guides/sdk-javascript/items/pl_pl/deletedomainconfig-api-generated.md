## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| domain | string | Yes |  |

## Odpowiedź

Zwraca: [`DeleteDomainConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteDomainConfigResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład deleteDomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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