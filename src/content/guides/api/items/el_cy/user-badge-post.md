Αυτό το endpoint σας επιτρέπει να δημιουργήσετε μια νέα ανάθεση σήματος χρήστη.

Παράδειγμα Αιτήματος:

[inline-code-attrs-start title = 'Παράδειγμα Αιτήματος POST'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X POST "https://fastcomments.com/api/v1/user-badges?tenantId=demo&API_KEY=DEMO_API_SECRET" \
-H "Content-Type: application/json" \
-d '{
  "userId": "user456",
  "badgeId": "badgeDef789",
  "displayedOnComments": true
}'
[inline-code-end]

Το σώμα του αιτήματος πρέπει να περιέχει τις ακόλουθες παραμέτρους:

- `userId` (απαιτούμενο) - Το ID του χρήστη στον οποίο θα ανατεθεί το σήμα
- `badgeId` (απαιτούμενο) - Το ID του σήματος που θα ανατεθεί
- `displayedOnComments` (προαιρετικό) - Αν το σήμα θα πρέπει να εμφανίζεται στα σχόλια του χρήστη (προεπιλογή true)

Σημαντικές Σημειώσεις:
1. Το σήμα πρέπει να υπάρχει και να είναι ενεργοποιημένο στον κατάλογο σημάτων του ενοικιαστή σας
2. Μπορείτε να αναθέσετε σήματα μόνο σε χρήστες που ανήκουν στον ενοικιαστή σας ή έχουν σχολιάσει στον ιστότοπό σας

Παράδειγμα Απάντησης:

[inline-code-attrs-start title = 'Απάντηση'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "userBadge": {
    "id": "badge123",
    "userId": "user456",
    "badgeId": "badgeDef789",
    "fromTenantId": "tenant001",
    "createdAt": 1650532511000,
    "receivedAt": 1650532511000,
    "type": 14,
    "name": "Special Contributor",
    "description": "Awarded to special contributors to our community",
    "displayLabel": "Special",
    "backgroundColor": "#4a5568",
    "textColor": "#ffffff",
    "displayedOnComments": true,
    "order": 1
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
  "reason": "User ID (body param: userId) is required."
}
[inline-code-end]

[inline-code-attrs-start title = 'Σφάλμα: Λείπει το Badge ID'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-badge-id",
  "reason": "Badge ID (body param: badgeId) is required."
}
[inline-code-end]

[inline-code-attrs-start title = 'Σφάλμα: Το Σήμα Δεν Βρέθηκε'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "badge-not-found",
  "reason": "The badge badgeDef789 was not found or is not enabled."
}
[inline-code-end]

[inline-code-attrs-start title = 'Σφάλμα: Μη Εξουσιοδοτημένος Χρήστης'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "unauthorized-user",
  "reason": "You can only add badges to users who belong to your tenant or have commented on your site."
}
[inline-code-end]
