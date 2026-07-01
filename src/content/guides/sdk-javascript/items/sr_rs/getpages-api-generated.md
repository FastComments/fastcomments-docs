## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |

## Odgovor

Vraća: [`GetPagesAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPagesAPIResponse.ts)

## Primer

[inline-code-attrs-start title = 'getPages Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";

(async () => {
  const response: GetPagesAPIResponse = await getPages(tenantId);
  const firstPage: APIPage | undefined = response.pages?.[0];
  console.log(firstPage?.title);
})();
[inline-code-end]