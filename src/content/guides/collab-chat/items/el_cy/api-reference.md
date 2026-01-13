### Επισκόπηση API

Το Collab Chat παρέχει τρία REST API τελικά σημεία για τη διαχείριση συνομιλιών κειμένου προγραμματιστικά. Αυτά τα τελικά σημεία σας επιτρέπουν να ανακτήσετε, να δημιουργήσετε και να διαγράψετε σημειώσεις χωρίς να χρησιμοποιήσετε το widget του προγράμματος περιήγησης.

Πρόκειται για δημόσια τελικά σημεία που αυθεντικοποιούν τους χρήστες μέσω των cookies του προγράμματος περιήγησης. Δεν χρησιμοποιούν κλειδιά API. Οι χρήστες πρέπει να είναι συνδεδεμένοι στο FastComments στον περιηγητή τους για να έχουν πρόσβαση σε αυτά τα τελικά σημεία.

### Βασικό URL

Όλα τα API τελικά σημεία του Collab Chat βρίσκονται κάτω από:

[inline-code-attrs-start title = 'Βασικό URL'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/comment-collab-chats
[inline-code-end]

### Έλεγχος ταυτότητας

Αυτά τα τελικά σημεία αυθεντικοποιούν χρήστες μέσω cookies του προγράμματος περιήγησης. Δεν χρησιμοποιούν κλειδιά API. Οι χρήστες πρέπει να είναι συνδεδεμένοι στο FastComments στον περιηγητή τους για να έχουν πρόσβαση σε αυτά τα τελικά σημεία.

### Λήψη όλων των συνομιλιών

Ανακτήστε όλες τις συνομιλίες κειμένου για μια συγκεκριμένη σελίδα.

#### Τελικό Σημείο

[inline-code-attrs-start title = 'GET Τελικό Σημείο'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET /comment-collab-chats/:tenantId?urlId=<urlId>
[inline-code-end]

#### Παράμετροι

`tenantId` (παράμετρος διαδρομής, απαραίτητη) είναι το FastComments Tenant ID σας.

`urlId` (παράμετρος ερωτήματος, απαραίτητη) είναι ο αναγνωριστής σελίδας για τον οποίο θέλετε να ανακτήσετε τις συνομιλίες.

#### Απόκριση

Η απόκριση περιλαμβάνει την κατάσταση του API, πληροφορίες για τη συνεδρία του τρέχοντος χρήστη εάν υπάρχει αυθεντικοποίηση, έναν πίνακα με συνομιλίες με τα αναγνωριστικά τους, τις διευθύνσεις URL και τα εύρη κειμένου, έναν καθαρισμένο αναγνωριστή URL, μια ένδειξη εάν ο τρέχων χρήστης είναι διαχειριστής ιστότοπου ή συντονιστής, καθώς και λεπτομέρειες σύνδεσης WebSocket για ζωντανή συγχρονισμό που περιλαμβάνουν `tenantIdWS`, `urlIdWS`, και `userIdWS`.

#### Παράδειγμα Αιτήματος

[inline-code-attrs-start title = 'Παράδειγμα Αιτήματος GET'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl "https://fastcomments.com/comment-collab-chats/demo?urlId=my-article-page" \
  --cookie "your-session-cookie"
[inline-code-end]

#### Παράδειγμα Απάντησης

[inline-code-attrs-start title = 'Παράδειγμα Απάντησης GET'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "user": {
    "id": "user123",
    "username": "john_doe"
  },
  "conversations": [
    {
      "_id": "conv123",
      "urlId": "my-article-page:p:0:15,0:45{abc123}",
      "range": "0:15,0:45{abc123}"
    },
    {
      "_id": "conv456",
      "urlId": "my-article-page:h1:5:20,5:35{def456}",
      "range": "5:20,5:35{def456}"
    }
  ],
  "urlIdClean": "my-article-page",
  "isSiteAdmin": false,
  "tenantIdWS": "demo",
  "urlIdWS": "my-article-page",
  "userIdWS": "user123"
}
[inline-code-end]

### Δημιουργία Συνομιλίας

Δημιουργήστε μια νέα συνομιλία κειμένου για μια συγκεκριμένη επιλογή κειμένου.

#### Τελικό Σημείο

[inline-code-attrs-start title = 'POST Τελικό Σημείο'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
POST /comment-collab-chats/:tenantId
[inline-code-end]

#### Παράμετροι

`tenantId` (παράμετρος διαδρομής, απαραίτητη) είναι το FastComments Tenant ID σας.

Το σώμα του αιτήματος πρέπει να είναι JSON και να περιλαμβάνει τα ακόλουθα απαιτούμενα πεδία.

`urlId` (string, απαραίτητο) είναι ο βασικός αναγνωριστής σελίδας.

`urlIdWithRange` (string, απαραίτητο) είναι το URL σε συνδυασμό με το εύρος κειμένου, για παράδειγμα `my-page:p:0:15,0:45{abc123}`.

`pageTitle` (string, απαραίτητο) είναι ο τίτλος της σελίδας.

`selector` (string, απαραίτητο) είναι η διαδρομή DOM προς το στοιχείο που περιέχει το επιλεγμένο κείμενο.

`range` (string, απαραίτητο) είναι το σειριοποιημένο εύρος κειμένου στη μορφή `startOffset:endOffset,startOffset:endOffset{checksum}`.

`createdFromCommentId` (string, απαραίτητο) είναι το ID του σχολίου που ξεκίνησε αυτή τη συνομιλία.

`broadcastId` (string, απαραίτητο) είναι ένα UUID για ζωντανό συγχρονισμό ώστε να αποφεύγονται τα φαινόμενα ηχούς.

#### Απόκριση

Η απόκριση περιλαμβάνει την κατάσταση του API και το ID της νεοδημιουργηθείσας συνομιλίας.

#### Παράδειγμα Αιτήματος

[inline-code-attrs-start title = 'Παράδειγμα Αιτήματος POST'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -X POST "https://fastcomments.com/comment-collab-chats/demo" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "urlId": "my-article-page",
    "urlIdWithRange": "my-article-page:p:0:15,0:45{abc123}",
    "pageTitle": "My Article Title",
    "selector": "div#article > p:nth-child(2)",
    "range": "0:15,0:45{abc123}",
    "createdFromCommentId": "comment789",
    "broadcastId": "550e8400-e29b-41d4-a716-446655440000"
  }'
[inline-code-end]

#### Παράδειγμα Απάντησης

[inline-code-attrs-start title = 'Παράδειγμα Απάντησης POST'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "createdChatId": "conv789"
}
[inline-code-end]

### Διαγραφή Συνομιλίας

Διαγράψτε μια υπάρχουσα συνομιλία κειμένου. Αυτό το τελικό σημείο απαιτεί δικαιώματα διαχειριστή ή συντονιστή, ή η συνομιλία πρέπει να έχει δημιουργηθεί από τον αυθεντικοποιημένο χρήστη.

#### Τελικό Σημείο

[inline-code-attrs-start title = 'DELETE Τελικό Σημείο'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
DELETE /comment-collab-chats/:tenantId/:chatId
[inline-code-end]

#### Παράμετροι

`tenantId` (παράμετρος διαδρομής, απαραίτητη) είναι το FastComments Tenant ID σας.

`chatId` (παράμετρος διαδρομής, απαραίτητη) είναι το ID της συνομιλίας που θέλετε να διαγράψετε.

`broadcastId` (σώμα αιτήματος, απαραίτητο) είναι ένα UUID για ζωντανό συγχρονισμό.

#### Παράδειγμα Αιτήματος

[inline-code-attrs-start title = 'Παράδειγμα Αιτήματος DELETE'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -X DELETE "https://fastcomments.com/comment-collab-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
[inline-code-end]

#### Παράδειγμα Απάντησης

[inline-code-attrs-start title = 'Παράδειγμα Απάντησης DELETE'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success"
}
[inline-code-end]

### Περιορισμός Ρυθμού

Αυτά τα τελικά σημεία υπόκεινται στα στάνταρ όρια ρυθμού του FastComments API. Το middleware περιορισμού ρυθμού εφαρμόζεται ανά tenant για να αποτραπεί η κατάχρηση ενώ επιτρέπεται η κανονική χρήση.

### Απαντήσεις Σφάλματος

Όλα τα τελικά σημεία επιστρέφουν τα τυπικά HTTP status codes. Μια απόκριση 400 υποδεικνύει μη έγκυρες παραμέτρους αιτήματος. Μια απόκριση 401 σημαίνει αποτυχία αυθεντικοποίησης. Μια απόκριση 403 υποδεικνύει ανεπαρκή δικαιώματα. Μια απόκριση 404 σημαίνει ότι η συνομιλία δεν βρέθηκε. Μια απόκριση 429 υποδεικνύει υπέρβαση του ορίου ρυθμού.

Οι απαντήσεις σφάλματος περιλαμβάνουν ένα JSON σώμα με λεπτομέρειες:

[inline-code-attrs-start title = 'Παράδειγμα Απάντησης Σφάλματος'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "error",
  "message": "Conversation not found"
}
[inline-code-end]

### Παρακολούθηση Χρήσης

Η δημιουργία συνομιλιών αυξάνει το μετρικό χρήση `conversationCreateCount`. Όλη η δραστηριότητα συγχρονισμού WebSocket αυξάνει τα `pubSubMessageCount` και `pubSubBandwidth`. Μπορείτε να παρακολουθείτε αυτά τα μετρικά στο πίνακα ελέγχου του FastComments κάτω από την ανάλυση χρήσης.