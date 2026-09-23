## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| page | number | Nie |  |

## Odpowiedź

Zwraca: [`GetHashTagsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetHashTagsResponse.ts)

## Przykład

[inline-code-attrs-start title = 'getHashTags Przykład'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
    const tenantId: string = "acme-corp-123";
    const page: number = 1;
    const result: GetHashTagsResponse = await getHashTags(tenantId, page);
    console.log(result);
})();
[inline-code-end]