### Επισκόπηση API

Το Collab Chat παρέχει τρία REST API σημεία τερματισμού για τη διαχείριση συνομιλιών κειμένου προγραμματιστικά. Αυτά τα σημεία τερματισμού σας επιτρέπουν να ανακτάτε, να δημιουργείτε και να διαγράφετε σημειώσεις χωρίς να χρησιμοποιείτε το widget του προγράμματος περιήγησης.

Πρόκειται για δημόσια σημεία τερματισμού που ταυτοποιούν τους χρήστες μέσω cookies του προγράμματος περιήγησης. Δεν χρησιμοποιούν API keys. Οι χρήστες πρέπει να έχουν συνδεθεί στο FastComments στο πρόγραμμα περιήγησής τους για να έχουν πρόσβαση σε αυτά τα σημεία τερματισμού.

### Βασικό URL

Όλα τα API σημεία τερματισμού του Collab Chat βρίσκονται κάτω από:

[inline-code-attrs-start title = 'Βασικό URL'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/comment-collab-chats
[inline-code-end]

### Αυθεντικοποίηση

Αυτά τα σημεία τερματισμού ταυτοποιούν τους χρήστες μέσω cookies του προγράμματος περιήγησης. Δεν χρησιμοποιούν API keys. Οι χρήστες πρέπει να έχουν συνδεθεί στο FastComments στο πρόγραμμα περιήγησής τους για να έχουν πρόσβαση σε αυτά τα σημεία τερματισμού.

### Ανάκτηση όλων των συνομιλιών

Ανάκτηση όλων των συνομιλιών κειμένου για μια συγκεκριμένη σελίδα.

#### Σημείο τερματισμού

[inline-code-attrs-start title = 'GET Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET /comment-collab-chats/:tenantId?urlId=<urlId>
[inline-code-end]

#### Παράμετροι

`tenantId` (path parameter, required) είναι το FastComments Tenant ID σας.

`urlId` (query parameter, required) είναι ο αναγνωριστής της σελίδας για την οποία θέλετε να ανακτήσετε τις συνομιλίες.

#### Απάντηση

Η απάντηση περιλαμβάνει την κατάσταση του API, πληροφορίες για την τρέχουσα συνεδρία χρήστη αν υπάρχει ταυτοποίηση, έναν πίνακα συνομιλιών με τα IDs τους, τις διευθύνσεις URL και τις σειρές κειμένου, έναν καθαρισμένο αναγνωριστή URL, μια σημαία που δείχνει αν ο τρέχων χρήστης είναι διαχειριστής ή moderator του site, και λεπτομέρειες σύνδεσης WebSocket για ζωντανή συγχρονισμό συμπεριλαμβανομένων των `tenantIdWS`, `urlIdWS`, και `userIdWS`.

#### Παράδειγμα Αιτήματος

[inline-code-attrs-start title = 'Παράδειγμα Αιτήματος GET'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl "https://fastcomments.com/comment-collab-chats/demo?urlId=my-article-page" \
  --cookie "your-session-cookie"
[inline-code-end]

#### Παράδειγμα Απάντησης

[inline-code-attrs-start title = 'GET Response Example'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Δημιουργία συνομιλίας

Δημιουργία μιας νέας συνομιλίας κειμένου για μια συγκεκριμένη επιλογή κειμένου.

#### Σημείο τερματισμού

[inline-code-attrs-start title = 'POST Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
POST /comment-collab-chats/:tenantId
[inline-code-end]

#### Παράμετροι

`tenantId` (path parameter, required) είναι το FastComments Tenant ID σας.

Το σώμα του αιτήματος πρέπει να είναι JSON και να περιλαμβάνει τα παρακάτω υποχρεωτικά πεδία.

`urlId` (string, required) είναι ο βασικός αναγνωριστής της σελίδας.

`urlIdWithRange` (string, required) είναι το URL σε συνδυασμό με τη σειρά κειμένου, για παράδειγμα `my-page:p:0:15,0:45{abc123}`.

`pageTitle` (string, required) είναι ο τίτλος της σελίδας.

`selector` (string, required) είναι το DOM path προς το στοιχείο που περιέχει το επιλεγμένο κείμενο.

`range` (string, required) είναι η σειριοποιημένη σειρά κειμένου στη μορφή `startOffset:endOffset,startOffset:endOffset{checksum}`.

`createdFromCommentId` (string, required) είναι το ID του σχολίου που ξεκίνησε αυτή τη συνομιλία.

`broadcastId` (string, required) είναι ένα UUID για ζωντανό συγχρονισμό ώστε να αποφεύγονται τα echo effects.

#### Απάντηση

Η απάντηση περιλαμβάνει την κατάσταση του API και το ID της νεοδημιουργηθείσας συνομιλίας.

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

[inline-code-attrs-start title = 'POST Response Example'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "createdChatId": "conv789"
}
[inline-code-end]

### Διαγραφή συνομιλίας

Διαγραφή μιας υπάρχουσας συνομιλίας κειμένου. Αυτό το σημείο τερματισμού απαιτεί δικαιώματα διαχειριστή ή moderator, ή η συνομιλία πρέπει να έχει δημιουργηθεί από τον ταυτοποιημένο χρήστη.

#### Σημείο τερματισμού

[inline-code-attrs-start title = 'DELETE Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
DELETE /comment-collab-chats/:tenantId/:chatId
[inline-code-end]

#### Παράμετροι

`tenantId` (path parameter, required) είναι το FastComments Tenant ID σας.

`chatId` (path parameter, required) είναι το ID της συνομιλίας που θέλετε να διαγράψετε.

`broadcastId` (request body, required) είναι ένα UUID για ζωντανό συγχρονισμό.

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

[inline-code-attrs-start title = 'DELETE Response Example'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success"
}
[inline-code-end]

### Περιορισμός Ρυθμού

Αυτά τα σημεία τερματισμού υπόκεινται στον τυπικό περιορισμό ρυθμού του FastComments API. Το middleware περιορισμού ρυθμού εφαρμόζεται ανά tenant για να αποτρέπει κατάχρηση ενώ επιτρέπει κανονικά μοτίβα χρήσης.

### Απαντήσεις Σφαλμάτων

Όλα τα σημεία τερματισμού επιστρέφουν τυπικούς HTTP κωδικούς κατάστασης. Μια απάντηση 400 δείχνει μη έγκυρες παραμέτρους αιτήματος. Μια απάντηση 401 σημαίνει ότι η αυθεντικοποίηση απέτυχε. Μια απάντηση 403 υποδεικνύει ανεπαρκή δικαιώματα. Μια απάντηση 404 σημαίνει ότι η συνομιλία δεν βρέθηκε. Μια απάντηση 429 υποδεικνύει υπέρβαση του ορίου ρυθμού.

Οι απαντήσεις σφάλματος περιλαμβάνουν ένα JSON σώμα με λεπτομέρειες:

[inline-code-attrs-start title = 'Παράδειγμα Απάντησης Σφάλματος'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "error",
  "message": "Conversation not found"
}
[inline-code-end]

### Παρακολούθηση χρήσης

Η δημιουργία συνομιλιών αυξάνει το usage metric `conversationCreateCount`. Όλη η δραστηριότητα συγχρονισμού WebSocket αυξάνει τα `pubSubMessageCount` και `pubSubBandwidth`. Μπορείτε να παρακολουθείτε αυτά τα metrics στο dashboard του FastComments κάτω από τα usage analytics.

---