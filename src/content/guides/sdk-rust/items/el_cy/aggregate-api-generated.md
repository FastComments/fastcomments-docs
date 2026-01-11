Συγκεντρώνει έγγραφα ομαδοποιώντας τα (εάν παρέχεται το groupBy) και εφαρμόζοντας πολλαπλές λειτουργίες.
Υποστηρίζονται διάφορες λειτουργίες (π.χ. sum, countDistinct, avg, κ.λπ.).

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenant_id | String | Ναι |  |
| aggregation_request | models::AggregationRequest | Ναι |  |
| parent_tenant_id | String | Όχι |  |
| include_stats | bool | Όχι |  |

## Απόκριση

Επιστρέφει: [`AggregationResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregation_response.rs)

---