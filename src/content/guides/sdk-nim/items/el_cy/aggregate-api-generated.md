Συγκεντρώνει έγγραφα ομαδοποιώντας τα (εάν παρέχεται το groupBy) και εφαρμόζοντας πολλαπλές λειτουργίες.  
Διαφορετικές λειτουργίες (π.χ. sum, countDistinct, avg, κ.λπ.) υποστηρίζονται.

## Parameters

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| aggregationRequest | AggregationRequest | No |  |
| options | AggregateOptions | No |  |

## Response

Επιστρέφει: [`Option[AggregateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_response.nim)

## Example

[inline-code-attrs-start title = 'Παράδειγμα Συγκέντρωσης'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (aggResp, httpResp) = client.aggregate(tenantId = "my-tenant-123", aggregationRequest = AggregationRequest(), options = AggregateOptions())
if aggResp.isSome:
  let response = aggResp.get()
  echo response
echo httpResp
[inline-code-end]