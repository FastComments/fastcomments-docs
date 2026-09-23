## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | No |  |

## Απάντηση

Επιστρέφει: [`BulkCreateHashTagsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BulkCreateHashTagsResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'addHashTagsBulk Παράδειγμα'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c6d";

const tags: BulkCreateHashTagsBodyTagsInner[] = [
  { name: "typescript", color: "#3178c6" },
  { name: "fastcomments", color: "#ff6600" }
];

const bulkBody: BulkCreateHashTagsBody = { tags };

const resultWithBody: BulkCreateHashTagsResponse = await addHashTagsBulk(tenantId, bulkBody);

const resultWithoutBody: BulkCreateHashTagsResponse = await addHashTagsBulk(tenantId);
[inline-code-end]