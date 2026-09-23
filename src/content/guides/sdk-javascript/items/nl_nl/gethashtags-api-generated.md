## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| page | number | Nee |  |

## Respons

Retourneert: [`GetHashTagsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetHashTagsResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getHashTags Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
    const tenantId: string = "acme-corp-123";
    const page: number = 1;
    const result: GetHashTagsResponse = await getHashTags(tenantId, page);
    console.log(result);
})();
[inline-code-end]

---