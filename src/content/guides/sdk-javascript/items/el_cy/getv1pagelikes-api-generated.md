## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| urlId | string | Ναι |  |

## Απόκριση

Επιστρέφει: [`GetV1PageLikes200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetV1PageLikes200Response.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getV1PageLikes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'fastcomments-742';
const urlId: string = 'sports/2026/06/15/world-cup-preview';
const pageLikes: GetV1PageLikes200Response = await getV1PageLikes(tenantId, urlId);
[inline-code-end]

---