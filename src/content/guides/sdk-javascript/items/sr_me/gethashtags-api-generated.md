## Parameters

| Ime | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| page | number | No |  |

## Response

Vraća: [`GetHashTagsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetHashTagsResponse1.ts)

## Example

[inline-code-attrs-start title = 'getHashTags Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";

  const responseWithPage: GetHashTagsResponse1 = await getHashTags(tenantId, 1);
  const responseDefault: GetHashTagsResponse1 = await getHashTags(tenantId);

  console.log(responseWithPage, responseDefault);
})();
[inline-code-end]

---