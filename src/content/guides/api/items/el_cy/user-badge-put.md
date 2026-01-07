Αυτό το endpoint σας επιτρέπει να ενημερώσετε μια ανάθεση σήματος χρήστη.

Αυτή τη στιγμή, η μόνη ιδιότητα που μπορεί να ενημερωθεί είναι το `displayedOnComments`, το οποίο ελέγχει αν το σήμα εμφανίζεται στα σχόλια του χρήστη.

Παράδειγμα Αιτήματος:

[inline-code-attrs-start title = 'Παράδειγμα Αιτήματος PUT'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X PUT "https://fastcomments.com/api/v1/user-badges/badge123?tenantId=demo&API_KEY=DEMO_API_SECRET" \
-H "Content-Type: application/json" \
-d '{
  "displayedOnComments": false
}'
[inline-code-end]

Παράδειγμα Απάντησης:

[inline-code-attrs-start title = 'Απάντηση'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success"
}
[inline-code-end]

Πιθανές Απαντήσεις Σφάλματος:

[inline-code-attrs-start title = 'Σφάλμα: Λείπει το Tenant ID'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-tenant-id",
  "reason": "Tenant ID (query param: tenantId) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = 'Σφάλμα: Λείπει το ID'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-id",
  "reason": "The User Badge ID (url param: id) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = 'Σφάλμα: Δεν Βρέθηκε'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "not-found",
  "reason": "The user badge badge123 was not found."
}
[inline-code-end]
