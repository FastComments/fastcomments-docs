## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| page | number | Nie |  |

## Odpowiedź

Zwraca: [`GetHashTags200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetHashTags200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getHashTags'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-7a9f";
  const tagsPage1: GetHashTags200Response = await getHashTags(tenantId);
  const tagsPage2: GetHashTags200Response = await getHashTags(tenantId, 2);
  console.log(tagsPage1, tagsPage2);
})();
[inline-code-end]

---