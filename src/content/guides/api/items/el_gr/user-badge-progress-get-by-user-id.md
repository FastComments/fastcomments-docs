Αυτό το endpoint σας επιτρέπει να ανακτήσετε την εγγραφή προόδου σήματος ενός χρήστη με το user ID του.

Παράδειγμα Αιτήματος:

[inline-code-attrs-start title = 'Παράδειγμα Αιτήματος GET'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X GET "https://fastcomments.com/api/v1/user-badge-progress/user/user456?tenantId=demo&API_KEY=DEMO_API_SECRET"
[inline-code-end]

Παράδειγμα Απάντησης:

[inline-code-attrs-start title = 'Απάντηση'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "userBadgeProgress": {
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
  }
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

[inline-code-attrs-start title = 'Σφάλμα: Λείπει το User ID'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-user-id",
  "reason": "The User ID (path param: userId) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = 'Σφάλμα: Δεν Βρέθηκε'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "not-found",
  "reason": "No badge progress found for user user456 in tenant tenant001."
}
[inline-code-end]
