Αυτό το endpoint σας επιτρέπει να ανακτήσετε σήματα χρηστών με βάση διάφορα κριτήρια.

Παράδειγμα Αιτήματος:

[inline-code-attrs-start title = 'Παράδειγμα Αιτήματος GET'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X GET "https://fastcomments.com/api/v1/user-badges?tenantId=demo&API_KEY=DEMO_API_SECRET"
[inline-code-end]

Μπορείτε να προσθέσετε διάφορες παραμέτρους ερωτήματος για να φιλτράρετε τα αποτελέσματα:

- `userId` - Λήψη σημάτων για συγκεκριμένο χρήστη
- `badgeId` - Λήψη περιπτώσεων συγκεκριμένου σήματος
- `type` - Φιλτράρισμα κατά τύπο σήματος (0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, κλπ. Δείτε τη δομή UserBadge για πλήρη λίστα)
- `displayedOnComments` - Φιλτράρισμα κατά το αν το σήμα εμφανίζεται στα σχόλια (true/false)
- `limit` - Μέγιστος αριθμός σημάτων που θα επιστραφούν (προεπιλογή 30, μέγιστο 200)
- `skip` - Αριθμός σημάτων που θα παραλειφθούν (για σελιδοποίηση)

Παράδειγμα Απάντησης:

[inline-code-attrs-start title = 'Απάντηση'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "userBadges": [
    {
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
    },
    {
      "id": "badge124",
      "userId": "user456",
      "badgeId": "badgeDef790",
      "fromTenantId": "tenant001",
      "createdAt": 1650532598000,
      "receivedAt": 1650532598000,
      "type": 0,
      "threshold": 100,
      "name": "Centurion",
      "description": "Made 100 comments",
      "displayLabel": "100",
      "backgroundColor": "#2b6cb0",
      "textColor": "#ffffff",
      "displayedOnComments": true,
      "order": 2
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

[inline-code-attrs-start title = 'Σφάλμα: Μη Έγκυρο Limit'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "invalid-limit",
  "reason": "The limit (query param: limit) is too large (> 200)."
}
[inline-code-end]
