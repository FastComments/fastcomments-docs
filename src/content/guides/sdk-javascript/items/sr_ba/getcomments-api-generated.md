## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| page | number | Ne |  |
| limit | number | Ne |  |
| skip | number | Ne |  |
| asTree | boolean | Ne |  |
| skipChildren | number | Ne |  |
| limitChildren | number | Ne |  |
| maxTreeDepth | number | Ne |  |
| urlId | string | Ne |  |
| userId | string | Ne |  |
| anonUserId | string | Ne |  |
| contextUserId | string | Ne |  |
| hashTag | string | Ne |  |
| parentId | string | Ne |  |
 
## Odgovor

Vraća: [`GetComments200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetComments200Response.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer getComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_42';
const response: GetComments200Response = await getComments(
  tenantId,
  1, // stranica
  20, // broj po strani
  0, // preskoči
  true, // kao stablo
  1, // preskoči djecu
  3, // ograničenje djece
  4, // maksimalna dubina stabla
  'articles/2026/new-product-launch', // ID URL-a
  'user_7890', // ID korisnika
  'anon_4f3b2', // anonimni ID korisnika
  undefined, // ID kontekstualnog korisnika
  '#launch', // hashtag
  undefined // ID roditelja
);
[inline-code-end]

---