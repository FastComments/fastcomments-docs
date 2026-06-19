## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| page | number | Не |  |

## Response

Враћа: [`GetHashTagsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetHashTagsResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример getHashTags'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-123";
const pageNumber: number = 2;
const responseWithPage: GetHashTagsResponse = await getHashTags(tenantId, pageNumber);
const responseFirstPage: GetHashTagsResponse = await getHashTags(tenantId);
[inline-code-end]

---