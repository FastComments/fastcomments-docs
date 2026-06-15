## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| title | string | Ne |  |

## Odgovor

Vraća: [`CreateV1PageReact200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateV1PageReact200Response.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer createV1PageReact'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'b12f3c4d-5678-90ab-cdef-1234567890ab';
  const urlId: string = 'https://www.news-site.com/world/2026/election-results';
  const title: string = 'Election results: key takeaways and analysis';
  const responseWithTitle: CreateV1PageReact200Response = await createV1PageReact(tenantId, urlId, title);
  const responseWithoutTitle: CreateV1PageReact200Response = await createV1PageReact(tenantId, urlId);
  console.log(responseWithTitle, responseWithoutTitle);
})();
[inline-code-end]