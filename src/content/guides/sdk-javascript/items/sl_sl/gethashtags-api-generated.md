## Parametri

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| page | number | Ne |  |

## Odziv

Vrne: [`GetHashTagsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetHashTagsResponse1.ts)

## Primer

[inline-code-attrs-start title = 'Primer getHashTags'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";

  const responseWithPage: GetHashTagsResponse1 = await getHashTags(tenantId, 1);
  const responseDefault: GetHashTagsResponse1 = await getHashTags(tenantId);

  console.log(responseWithPage, responseDefault);
})();
[inline-code-end]