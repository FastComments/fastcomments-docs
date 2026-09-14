FastComments είναι ένας διακομιστής εξουσιοδότησης OAuth 2.1. Μια εφαρμογή μπορεί να λάβει ένα token που συνδέεται με έναν λογαριασμό FastComments και να το χρησιμοποιήσει σε κάθε σημείο τερματισμού σε αυτόν τον οδηγό αντί για κλειδί API. Έτσι συνδέονται η εφαρμογή Zapier, ο διακομιστής MCP και άλλες ενσωματώσεις τρίτων.

Τα tokens εκδίδονται μέσω της ροής κώδικα εξουσιοδότησης με PKCE. Δεν υπάρχει παροχή διαπιστευτηρίων πελάτη ή έμμεση χορήγηση.

### Ανακάλυψη

Οι τοποθεσίες των σημείων τερματισμού, τα υποστηριζόμενα grants και οι μέθοδοι εξουσιοδότησης δημοσιεύονται στη στάνταρ διεύθυνση μεταδεδομένων:

[inline-code-attrs-start title = 'Μεταδεδομένα Διακομιστή Εξουσιοδότησης'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Τα σημεία τερματισμού που περιγράφει:

[inline-code-attrs-start title = 'Σημεία Τερματισμού OAuth'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET  https://fastcomments.com/oauth/authorize
POST https://fastcomments.com/oauth/token
POST https://fastcomments.com/oauth/revoke
POST https://fastcomments.com/oauth/register
[inline-code-end]

Οι λογαριασμοί στην περιοχή EU χρησιμοποιούν το `https://eu.fastcomments.com` ως εκδότη, με τις ίδιες διαδρομές.

### Καταχώρηση πελάτη

Ένας πελάτης χρειάζεται ένα `client_id` και μια καταχωρημένη `redirect_uri` πριν μπορέσει να ξεκινήσει τη ροή. Υπάρχουν δύο τρόποι για να το αποκτήσει:

- **Dynamic Client Registration.** `POST /oauth/register` με σώμα JSON σύμφωνα με το RFC 7591 (`redirect_uris`, `client_name`, `client_uri`, `logo_uri`, `token_endpoint_auth_method`). Η απάντηση περιέχει το `client_id` και, για εμπιστευτικούς πελάτες, το `client_secret`. Η καταχώρηση είναι χωρίς αυθεντικοποίηση και περιορίζεται ανά IP.
- **Client ID Metadata Document.** Ο πελάτης χρησιμοποιεί ένα URL `https` που ελέγχει ως το `client_id` του. Το FastComments ανακτά αυτό το URL και διαβάζει τα ίδια πεδία μεταδεδομένων από αυτό. Δεν απαιτείται κλήση καταχώρησης.

Οι εφαρμογές συνεργατών που εμφανίζονται στον πίνακα ελέγχου του FastComments, όπως το Zapier, καταχωρούνται απευθείας από το FastComments. Επικοινωνήστε με την υποστήριξη εάν δημιουργείτε μια καταχώρηση στην αγορά και χρειάζεστε πελάτη πρώτου μέρους.

### Δυνατότητες

[inline-code-attrs-start title = 'Δυνατότητες'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
read   GET and HEAD requests
write  every other method
[inline-code-end]

Ένα αίτημα που δεν ζητά καμία δυνατότητα χορηγείται και τα δύο. Ο χρήστης βλέπει τις ζητούμενες δυνατότητες στη σελίδα συναίνεσης. Ένα αίτημα για δυνατότητα διαφορετική από αυτές τις δύο αποτυγχάνει με `invalid_scope`.

### Βήμα 1 - Αίτηση εξουσιοδότησης

Στείλτε τον περιηγητή του χρήστη στο σημείο τερματισμού εξουσιοδότησης. Το PKCE με τη μέθοδο `S256` απαιτείται για κάθε πελάτη.

[inline-code-attrs-start title = 'Αίτηση Εξουσιοδότησης'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET https://fastcomments.com/oauth/authorize
    ?response_type=code
    &client_id=YOUR_CLIENT_ID
    &redirect_uri=https://example.com/oauth/callback
    &scope=read%20write
    &state=RANDOM_STATE
    &code_challenge=BASE64URL_SHA256_OF_VERIFIER
    &code_challenge_method=S256
[inline-code-end]

[inline-code-attrs-start title = 'Παράμετροι Αίτησης Εξουσιοδότησης'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthAuthorizeQueryParams {
    response_type: 'code'
    client_id: string
    /** Πρέπει να ταιριάζει ακριβώς με μία από τις καταχωρημένες URI ανακατεύθυνσης του πελάτη. **/
    redirect_uri: string
    /** Διαχωρίζονται με κενό. Παραλείψτε για να ζητήσετε και τις δύο δυνατότητες. **/
    scope?: 'read' | 'write' | 'read write'
    /** Επιστρέφεται αμετάβλητο στην ανακατεύθυνση. Χρησιμοποιήστε το για να συνδέσετε την κλήση επιστροφής με τη συνεδρία που ξεκίνησε τη ροή. **/
    state?: string
    /** base64url(sha256(code_verifier)). **/
    code_challenge: string
    code_challenge_method: 'S256'
    /** Προαιρετικός δείκτης πόρου RFC 8707. Εάν αποσταλεί, η ίδια τιμή πρέπει να αποσταλεί στο σημείο τερματισμού token. **/
    resource?: string
}
[inline-code-end]

Ο χρήστης συνδέεται στο FastComments εάν χρειάζεται και βλέπει μια σελίδα συναίνεσης που ονομάζει την εφαρμογή σας, τον λογαριασμό στον οποίο θα συνδεθεί, και τις ζητούμενες δυνατότητες. Ο χρήστης πρέπει να κατέχει το δικαίωμα **API Admin** σε αυτόν τον λογαριασμό· οποιοσδήποτε άλλος βλέπει σφάλμα δικαιωμάτων αντί για τη φόρμα συναίνεσης. Η έγκριση ανακατευθύνει τον περιηγητή στο `redirect_uri` σας με `code` και `state`. Η άρνηση ανακατευθύνει με `error=access_denied`.

Ο κωδικός εξουσιοδότησης ισχύει για 10 λεπτά και μπορεί να ανταλλαχθεί μία φορά. Μια δεύτερη ανταλλαγή του ίδιου κωδικού ανακαλεί κάθε token που παρήγαγε η πρώτη ανταλλαγή.

### Βήμα 2 - Αίτηση token

Ανταλλάξτε τον κωδικό για tokens. Το σώμα είναι κωδικοποιημένο ως φόρμα. Οι εμπιστευτικοί πελάτες πιστοποιούνται με `client_secret_basic` (HTTP Basic) ή `client_secret_post` (μυστικό στο σώμα). Οι δημόσιοι πελάτες στέλνουν μόνο το `client_id`.

[inline-code-attrs-start title = 'Παράδειγμα cURL Αίτησης Token'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/token' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'grant_type=authorization_code' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'code=fcac_...' \
  --data 'code_verifier=YOUR_PKCE_VERIFIER' \
  --data 'redirect_uri=https://example.com/oauth/callback'
[inline-code-end]

[inline-code-attrs-start title = 'Σώμα Αίτησης Token (authorization_code)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenRequestAuthorizationCode {
    grant_type: 'authorization_code'
    client_id: string
    /** Μόνο για εμπιστευτικούς πελάτες. Μπορεί να αποσταλεί ως HTTP Basic authentication αντί αυτού. **/
    client_secret?: string
    code: string
    code_verifier: string
    /** Πρέπει να ταιριάζει με το αίτημα εξουσιοδότησης όταν αποστέλλεται. **/
    redirect_uri?: string
    resource?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απόκρισης Token'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenResponse {
    /** Με πρόθεμα fcat_. Ισχύει για μία ώρα. **/
    access_token: string
    token_type: 'bearer'
    /** Δευτερόλεπτα μέχρι τη λήξη του access token. 3600. **/
    expires_in: number
    /** Με πρόθεμα fcrt_. Ισχύει για 30 ημέρες από την έκδοση. **/
    refresh_token: string
    /** Δυνατότητες που χορηγούνται, διαχωρισμένες με κενό. **/
    scope: string
}
[inline-code-end]

Τα σφάλματα ακολουθούν το RFC 6749: ένα σώμα JSON με `error` και `error_description`, HTTP 400 για `invalid_request`, `invalid_grant`, `invalid_scope`, `invalid_target` και `unsupported_grant_type`, HTTP 401 για `invalid_client`, HTTP 429 όταν υπάρχει περιορισμός ταχύτητας.

### Βήμα 3 - Κλήση του API

Στείλτε το access token ως token τύπου bearer. Ο ενοικιαστής (tenant) υπονοείται από το token, έτσι το `tenantId` είναι προαιρετικό. Όταν παρέχεται, πρέπει να ταιριάζει με το token ή το αίτημα αποτυγχάνει.

[inline-code-attrs-start title = 'Παράδειγμα cURL Bearer Token'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me' \
  --header 'Authorization: Bearer fcat_...'
[inline-code-end]

`GET /api/v1/me` επιστρέφει τον ενοικιαστή, τον εξουσιοδοτημένο χρήστη και τις χορηγμένες δυνατότητες, κάτι που το καθιστά το κατάλληλο αίτημα για δοκιμή σύνδεσης. Ένα αίτημα με ληγμένο ή ανακληθέν token λαμβάνει HTTP 401. Ένα αίτημα του οποίου η μέθοδος απαιτεί δυνατότητα που δεν κατέχει το token λαμβάνει HTTP 403.

### Βήμα 4 - Ανανέωση

[inline-code-attrs-start title = 'Παράδειγμα cURL Αίτησης Ανανέωσης'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/token' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'grant_type=refresh_token' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'refresh_token=fcrt_...'
[inline-code-end]

[inline-code-attrs-start title = 'Σώμα Αίτησης Token (refresh_token)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenRequestRefreshToken {
    grant_type: 'refresh_token'
    client_id: string
    client_secret?: string
    refresh_token: string
    /** Προαιρετικό. Περιορίζει σε ένα υποσύνολο των αρχικά χορηγμένων δυνατοτήτων. **/
    scope?: string
    resource?: string
}
[inline-code-end]

Η απόκριση έχει την ίδια δομή με την ανταλλαγή κώδικα. Τα refresh tokens περιστρέφονται: κάθε ανανέωση επιστρέφει ένα νέο `refresh_token` και ανακαλεί το παλιό μετά από παράθυρο χάριτος 30 δευτερολέπτων για ταυτόχρονες κλήσεις. Η παρουσίαση ενός refresh token που έχει περιστραφεί περισσότερο από 30 δευτερόλεπτα θεωρείται επανάληψη και ανακαλεί ολόκληρη τη χορήγηση. Οι εφαρμογές συνεργατών που καταχωρούνται από το FastComments εξαιρούνται από την περιστροφή και λαμβάνουν το ίδιο refresh token με την ημερομηνία λήξης επεκταμένη κατά ακόμη 30 ημέρες.

Μια ανανέωση ελέγχει επίσης ότι ο εξουσιοδοτών χρήστης εξακολουθεί να κατέχει το API Admin στον λογαριασμό. Εάν όχι, η χορήγηση ανακαλείται και η απόκριση είναι `invalid_grant`.

### Ανάκληση

[inline-code-attrs-start title = 'Παράδειγμα cURL Αίτησης Ανάκλησης'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/revoke' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'token=fcrt_...' \
  --data 'token_type_hint=refresh_token'
[inline-code-end]

Η ανάκληση ενός refresh token ανακαλεί κάθε access token που εκδόθηκε από την ίδια χορήγηση. Η ανάκληση ενός access token ανακαλεί μόνο αυτό το token. Το σημείο τερματισμού επιστρέφει HTTP 200 με ένα κενό αντικείμενο JSON, ανεξάρτητα από το αν βρέθηκε το token, σύμφωνα με το RFC 7009.

Οι χρήστες μπορούν επίσης να ανακαλέσουν μια σύνδεση από τις **Συνδεδεμένες Εφαρμογές** στον πίνακα ελέγχου του FastComments. Κάθε token για αυτήν την εφαρμογή σταματά να λειτουργεί αμέσως.