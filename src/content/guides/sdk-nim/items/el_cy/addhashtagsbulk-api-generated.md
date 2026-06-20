## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | Όχι |  |

## Απόκριση

Επιστρέφει: [`Option[BulkCreateHashTagsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_bulk_create_hash_tags_response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'addHashTagsBulk Παράδειγμα'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.addHashTagsBulk(tenantId = "my-tenant-123", bulkCreateHashTagsBody = BulkCreateHashTagsBody(hashTags = @["news", "breaking", "politics"], replaceExisting = false))
if response.isSome:
  let result = response.get()
  echo "Bulk tags response:", result
else:
  echo "No response body, HTTP status:", httpResponse.statusCode
[inline-code-end]

---