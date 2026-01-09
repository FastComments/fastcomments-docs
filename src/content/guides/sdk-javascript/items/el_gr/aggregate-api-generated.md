Συγκεντρώνει έγγραφα ομαδοποιώντας τα (αν παρέχεται το groupBy) και εφαρμόζοντας πολλαπλές λειτουργίες. Υποστηρίζονται διάφορες λειτουργίες (π.χ. sum, countDistinct, avg, κ.λπ.).

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| aggregationRequest | AggregationRequest | Ναι |  |
| parentTenantId | string | Όχι |  |
| includeStats | boolean | Όχι |  |

## Απόκριση

Επιστρέφει: [`AggregationResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregationResponse.ts)

---