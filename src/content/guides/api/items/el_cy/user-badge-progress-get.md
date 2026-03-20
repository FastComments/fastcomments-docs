Αυτό το τελικό σημείο σας επιτρέπει να ανακτήσετε εγγραφές προόδου κονκάρδων χρηστών με βάση διάφορα κριτήρια.

Παράδειγμα Αιτήματος:

[inline-code-attrs-start title = 'Λίστα Προόδου Κονκάρδων - Παράδειγμα GET'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X GET "https://fastcomments.com/api/v1/user-badge-progress?tenantId=demo&API_KEY=DEMO_API_SECRET"
[inline-code-end]

Μπορείτε να προσθέσετε διάφορες παραμέτρους ερωτήματος για να φιλτράρετε τα αποτελέσματα:

- `userId` - Λήψη προόδου για συγκεκριμένο χρήστη
- `limit` - Μέγιστος αριθμός εγγραφών προς επιστροφή (προεπιλογή 30, max 200)
- `skip` - Αριθμός εγγραφών προς παράλειψη (για σελιδοποίηση)

Παράδειγμα Απόκρισης:

[inline-code-attrs-start title = 'Απόκριση'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "userBadgeProgresses": [
    {
      "id": "progress123",
      "tenantId": "tenant001",
      "userId": "user456",
      "firstCommentId": "comment789",
      "firstCommentDate": 1650532511000,
      "autoTrustFactor": 0.75,
      "progress": {
        "0": 42,
        "1": 120,
        "2": 15,
        "3": 3,
        "5": 5,
        "6": 1800000,
        "8": 0,
        "7": 0
      }
    },
    {
      "id": "progress124",
      "tenantId": "tenant001",
      "userId": "user789",
      "firstCommentId": "comment790",
      "firstCommentDate": 1650532598000,
      "autoTrustFactor": 0.5,
      "progress": {
        "0": 12,
        "1": 15,
        "2": 4
      }
    }
  ]
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

[inline-code-attrs-start title = 'Σφάλμα: Άκυρο Limit'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "invalid-limit",
  "reason": "The limit (query param: limit) is too large (> 200)."
}
[inline-code-end]