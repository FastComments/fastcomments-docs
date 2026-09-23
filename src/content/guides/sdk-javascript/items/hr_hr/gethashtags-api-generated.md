## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| page | number | Ne |  |

## Odgovor

Returns: [`GetHashTagsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetHashTagsResponse.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer getHashTags'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
    const tenantId: string = "acme-corp-123";
    const page: number = 1;
    const result: GetHashTagsResponse = await getHashTags(tenantId, page);
    console.log(result);
})();
[inline-code-end]