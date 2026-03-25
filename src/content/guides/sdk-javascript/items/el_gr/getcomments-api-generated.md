## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| page | number | Όχι |  |
| limit | number | Όχι |  |
| skip | number | Όχι |  |
| asTree | boolean | Όχι |  |
| skipChildren | number | Όχι |  |
| limitChildren | number | Όχι |  |
| maxTreeDepth | number | Όχι |  |
| urlId | string | Όχι |  |
| userId | string | Όχι |  |
| anonUserId | string | Όχι |  |
| contextUserId | string | Όχι |  |
| hashTag | string | Όχι |  |
| parentId | string | Όχι |  |
| direction | SortDirections | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetComments200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetComments200Response.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_42';
const response: GetComments200Response = await getComments(
  tenantId,
  1, // σελίδα
  20, // όριο
  0, // παράλειψη
  true, // ως δέντρο
  1, // παράλειψη παιδιών
  3, // όριο παιδιών
  4, // μέγιστο βάθος δέντρου
  'articles/2026/new-product-launch', // αναγνωριστικό διεύθυνσης URL
  'user_7890', // αναγνωριστικό χρήστη
  'anon_4f3b2', // ανώνυμο αναγνωριστικό χρήστη
  undefined, // αναγνωριστικό χρήστη πλαισίου
  '#launch', // hashtag
  undefined // αναγνωριστικό γονέα
);
[inline-code-end]