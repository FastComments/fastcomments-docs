Σε αυτό το κείμενο θα δούμε βήμα προς βήμα την κλήση του FastComments API για τη ρύθμιση του ελέγχου πρόσβασης.

Πριν ξεκινήσουμε, σημειώστε ότι δεν χρειάζεται να δημιουργήσουμε ρητά μια δομή `Group`. Οι ομάδες είναι απλώς αναγνωριστικά που προστίθενται σε `Users` και `Pages`. Η προσθήκη μιας ομάδας σε χρήστη ή σελίδα "δημιουργεί" αυτόματα την ομάδα.

Πρώτα, ας δημιουργήσουμε δύο χρήστες, `User A` και `User B`, οι οποίοι θα ξεκινήσουν στην `Group X`:

[inline-code-attrs-start title = 'Παράδειγμα cURL για δημιουργία του User A'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "user-a",
	"username": "User A",
	"email": "usera@example.com",
    "groupIds": ["GROUP-X"]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Παράδειγμα cURL για δημιουργία του User B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "user-b",
	"username": "User B",
	"email": "userb@example.com",
    "groupIds": ["GROUP-X"]
}'
[inline-code-end]

Τώρα ας δημιουργήσουμε μια `Page`. Θα την ονομάσουμε `Confidential Page`, και προς το παρόν κανένας από αυτούς τους χρήστες δεν θα έχει πρόσβαση σε αυτήν, καθώς θα βρίσκεται στην ομάδα `CONFIDENTIAL`:

[inline-code-attrs-start title = 'Παράδειγμα cURL POST για σελίδα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"title": "Confidential Page",
	"url": "https://mysite.com/confidential",
	"urlId": "https://mysite.com/confidential",
	"accessibleByGroupIds": ["CONFIDENTIAL"]
}'
[inline-code-end]

Οι χρήστες A και B προς το παρόν **ΔΕΝ** έχουν πρόσβαση στη νέα σελίδα. Ωστόσο, καθώς βρίσκονται στην ίδια ομάδα, `GROUP-X`, μπορούν να κάνουν `@mention` ο ένας στον άλλον.

Ας ενημερώσουμε τον `User B` ώστε να έχει πλέον πρόσβαση στη σελίδα:

[inline-code-attrs-start title = 'Παράδειγμα cURL για ενημέρωση του User B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["GROUP-X", "CONFIDENTIAL"]
}'
[inline-code-end]

Ο `User B` ανήκει πλέον και στις δύο ομάδες. Οι χρήστες μας μπορούν ακόμα να κάνουν `@mention` ο ένας στον άλλον, αλλά μόνο ο `User B` μπορεί να δει τη εμπιστευτική σελίδα.

Ας κάνουμε τον `User B` να μπορεί να βλέπει μόνο την εμπιστευτική σελίδα:

[inline-code-attrs-start title = 'Παράδειγμα cURL για ενημέρωση του User B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["CONFIDENTIAL"]
}'
[inline-code-end]

Τώρα μπορεί να δει την εμπιστευτική σελίδα, αλλά κανείς από τους δύο χρήστες δεν μπορεί να κάνει `@mention` στον άλλον, καθώς βρίσκονται σε διαφορετικές ομάδες.

Ωστόσο, κάθε χρήστης που δεν ανήκει στον έλεγχο πρόσβασης **θα μπορεί να έχει πρόσβαση στη σελίδα μας**. Για να το αποτρέψουμε αυτό, βεβαιωθείτε ότι κανένας SSO User δεν έχει τα `groupIds` του ορισμένα σε null. Για παράδειγμα, ας δημιουργήσουμε τον `User C`, ο οποίος έχει πρόσβαση σε όλα:

[inline-code-attrs-start title = 'Παράδειγμα cURL για δημιουργία του User C'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "user-c",
	"username": "User C",
	"email": "userc@example.com",
    "groupIds": null
}'
[inline-code-end]

Ορίζοντας τα `groupIds` σε null, δηλώνουμε ότι δεν περιορίζεται από τον έλεγχο πρόσβασης.

Τώρα, ας δημιουργήσουμε μια σελίδα στην οποία έχουν πρόσβαση όλοι:

[inline-code-attrs-start title = 'Παράδειγμα cURL POST για σελίδα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"title": "Public Page",
	"url": "https://mysite.com/public",
	"urlId": "https://mysite.com/public",
	"accessibleByGroupIds": null
}'
[inline-code-end]

Ορίζοντας τα `accessibleByGroupIds` σε null, δηλώνουμε ότι αυτή η `Page` δεν ελέγχεται μέσω του ελέγχου πρόσβασης, και οι δύο χρήστες μπορούν να έχουν πρόσβαση σε αυτήν.

Με αυτό ολοκληρώνεται η παρουσίαση του API για τον Έλεγχο Πρόσβασης.