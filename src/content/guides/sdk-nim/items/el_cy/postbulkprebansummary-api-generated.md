## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|------------|
| tenantId | string | Ναι |  |
| bulkPreBanParams | BulkPreBanParams | Όχι |  |
| options | PostBulkPreBanSummaryOptions | Όχι |  |

## Απάντηση

Επιστρέφει: [`Option[BulkPreBanSummary]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_bulk_pre_ban_summary.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'postBulkPreBanSummary Παράδειγμα'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let bulkParams = BulkPreBanParams(emailAddresses: @["spam@example.com"], ipAddresses: @["203.0.113.5"])
let opt = PostBulkPreBanSummaryOptions(dryRun: false, maxResults: 0)
let (summaryOpt, httpResp) = client.postBulkPreBanSummary(tenantId = "my-tenant-123", bulkPreBanParams = bulkParams, options = opt)
if summaryOpt.isSome:
  let _ = summaryOpt.get()
[inline-code-end]

---