Μετά τη ρύθμιση του SAML στο FastComments, πρέπει να ρυθμίσετε το FastComments ως Πάροχο Υπηρεσίας στον πάροχο ταυτότητάς σας.

### Γενική Διαμόρφωση IdP

Οι περισσότεροι πάροχοι ταυτότητας απαιτούν τις παρακάτω πληροφορίες για να προσθέσουν το FastComments ως εφαρμογή SAML:

#### Απαραίτητες Πληροφορίες Πάροχου Υπηρεσίας

Οι τιμές αυτές δημιουργούνται αυτόματα και εμφανίζονται στη σελίδα διαμόρφωσης SAML του FastComments:

**SP Entity ID / Audience**
- Format: `https://fastcomments.com/saml/{your-tenant-id}`
- Αυτό αναγνωρίζει μοναδικά την εγκατάσταση FastComments σας

**Assertion Consumer Service (ACS) URL**
- Format: `https://fastcomments.com/saml/callback/{your-tenant-id}`
- Όπου ο IdP σας αποστέλλει απαντήσεις SAML μετά την αυθεντικοποίηση

**SP Metadata URL** *(if supported by your IdP)*
- Format: `https://fastcomments.com/saml/metadata/{your-tenant-id}`
- Παρέχει πλήρη διαμόρφωση SAML σε μορφή XML

**SAML Login URL**
- Format: `https://fastcomments.com/saml/login/{your-tenant-id}`
- Άμεσος σύνδεσμος για την έναρξη της αυθεντικοποίησης SAML

### Απαιτούμενα Χαρακτηριστικά SAML

Ρυθμίστε τον πάροχο ταυτότητάς σας ώστε να στέλνει αυτά τα χαρακτηριστικά με τις απαντήσεις SAML:

#### Βασικά Χαρακτηριστικά

**Email Address** *(Required)*
- **Attribute Name**: `email`, `emailAddress`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress`
- **Purpose**: Μοναδική αναγνώριση χρήστη και ειδοποιήσεις
- **Format**: Έγκυρη διεύθυνση email

#### Προαιρετικά Χαρακτηριστικά

**First Name**
- **Attribute Names**: `firstName`, `givenName`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/givenname`
- **Purpose**: Όνομα εμφάνισης χρήστη

**Last Name**
- **Attribute Names**: `lastName`, `surname`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/surname`
- **Purpose**: Επώνυμο για εμφάνιση χρήστη

**Roles** *(Important for access control)*
- **Attribute Names**: `roles`, `groups`, `memberOf`, or custom attribute names
- **Purpose**: Ανάθεση ρόλων και δικαιωμάτων στο FastComments
- **Format**: Πίνακας συμβολοσειρών ρόλων ή τιμές χωρισμένες με κόμμα

### Κοινές Ρυθμίσεις Παρόχων Ταυτότητας

#### Microsoft Azure AD

1. **Add Enterprise Application**
   - Αναζητήστε "FastComments" ή δημιουργήστε μια προσαρμοσμένη εφαρμογή SAML
   - Χρησιμοποιήστε τις πληροφορίες SP που παρέχονται από το FastComments

2. **Configure Attributes**
   - Email: `user.mail` or `user.userprincipalname`
   - First Name: `user.givenname`
   - Last Name: `user.surname`
   - Roles: `user.assignedroles` or directory groups

#### Okta

1. **Create SAML Application**
   - Χρησιμοποιήστε "Create New App" και επιλέξτε SAML 2.0
   - Διαμορφώστε με τις πληροφορίες SP του FastComments

2. **Attribute Statements**
   - Email: `user.email`
   - FirstName: `user.firstName`
   - LastName: `user.lastName`
   - Roles: `user.groups` or custom attributes

#### Google Workspace

1. **Add SAML Application**
   - Μεταβείτε σε Apps > Web and mobile apps > Add App > Add custom SAML app
   - Διαμορφώστε με τις πληροφορίες SP του FastComments

2. **Attribute Mapping**
   - Email: Primary email
   - First Name: First name
   - Last Name: Last name
   - Roles: Groups or custom attributes

#### Active Directory Federation Services (ADFS)

1. **Add Relying Party Trust**
   - Χρησιμοποιήστε το URL μεταδεδομένων του FastComments ή χειροκίνητη διαμόρφωση
   - Διαμορφώστε τις πληροφορίες SP όπως παρέχονται

2. **Claim Rules**
   - Email: Email Address claim
   - Name: Name ID claim
   - Roles: Group membership or custom claims

### Ευελιξία Ονομάτων Χαρακτηριστικών

Το FastComments δέχεται πληροφορίες ρόλων από πολλαπλά ονόματα χαρακτηριστικών για να υποστηρίξει διαφορετικές ρυθμίσεις IdP:

- `roles`
- `groups`
- `memberOf`
- `role`
- `group`
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

Αυτή η ευελιξία εξασφαλίζει συμβατότητα με διάφορους παρόχους ταυτότητας χωρίς να απαιτούνται συγκεκριμένες ονομασίες χαρακτηριστικών.

### Δοκιμή της Διαμόρφωσής Σας

Μετά τη ρύθμιση του παρόχου ταυτότητας:

1. Αποθηκεύστε τη διαμόρφωση IdP
2. Δοκιμάστε με έναν αφιερωμένο λογαριασμό δοκιμών
3. Επιβεβαιώστε ότι τα χαρακτηριστικά αποστέλλονται σωστά
4. Ελέγξτε ότι οι ρόλοι έχουν χαρτογραφηθεί σωστά
5. Βεβαιωθείτε ότι η ροή αυθεντικοποίησης ολοκληρώνεται επιτυχώς

Οι περισσότεροι πάροχοι ταυτότητας προσφέρουν εργαλεία δοκιμής SAML για την επικύρωση της διαμόρφωσης πριν την ανάπτυξη σε χρήστες παραγωγής.