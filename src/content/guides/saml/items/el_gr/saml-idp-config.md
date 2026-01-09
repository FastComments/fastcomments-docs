Μετά τη διαμόρφωση του SAML στο FastComments, χρειάζεται να ρυθμίσετε το FastComments ως Πάροχο Υπηρεσίας στο σύστημα του παρόχου ταυτότητας σας.

### Γενική Ρύθμιση IdP

Οι περισσότεροι πάροχοι ταυτότητας απαιτούν τις παρακάτω πληροφορίες για να προσθέσουν το FastComments ως εφαρμογή SAML:

#### Απαραίτητες Πληροφορίες Πάροχου Υπηρεσίας

Αυτές οι τιμές δημιουργούνται αυτόματα και εμφανίζονται στη σελίδα διαμόρφωσης SAML του FastComments:

**SP Entity ID / Audience**
- Format: `https://fastcomments.com/saml/{your-tenant-id}`
- Αυτό αναγνωρίζει μοναδικά το instance του FastComments σας

**Assertion Consumer Service (ACS) URL**
- Format: `https://fastcomments.com/saml/callback/{your-tenant-id}`
- Εκεί όπου ο IdP στέλνει τις SAML απαντήσεις μετά την πιστοποίηση

**SP Metadata URL** *(αν υποστηρίζεται από τον IdP σας)*
- Format: `https://fastcomments.com/saml/metadata/{your-tenant-id}`
- Παρέχει πλήρη SAML διαμόρφωση σε μορφή XML

**SAML Login URL**
- Format: `https://fastcomments.com/saml/login/{your-tenant-id}`
- Άμεσος σύνδεσμος για την έναρξη της SAML πιστοποίησης

### Απαιτούμενα SAML Attributes

Διαμορφώστε τον πάροχο ταυτότητας σας ώστε να αποστέλλει αυτά τα attributes με τις SAML απαντήσεις:

#### Θεμελιώδη Attributes

**Email Address** *(Απαραίτητο)*
- **Attribute Name**: `email`, `emailAddress`, ή `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress`
- **Σκοπός**: Μοναδική αναγνώριση χρήστη και ειδοποιήσεις
- **Format**: Έγκυρη διεύθυνση email

#### Προαιρετικά Attributes

**First Name**
- **Attribute Names**: `firstName`, `givenName`, ή `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/givenname`
- **Σκοπός**: Όνομα για εμφάνιση χρήστη

**Last Name**
- **Attribute Names**: `lastName`, `surname`, ή `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/surname`
- **Σκοπός**: Επώνυμο για εμφάνιση χρήστη

**Roles** *(Σημαντικό για έλεγχο πρόσβασης)*
- **Attribute Names**: `roles`, `groups`, `memberOf`, ή προσαρμοσμένα ονόματα attributes
- **Σκοπός**: Ανάθεση ρόλων και δικαιωμάτων στο FastComments
- **Format**: Πίνακας με string ρόλων ή τιμές χωρισμένες με κόμμα

### Συνηθισμένες Ρυθμίσεις Παρόχων Ταυτότητας

#### Microsoft Azure AD

1. **Προσθήκη Enterprise Application**
   - Αναζητήστε "FastComments" ή δημιουργήστε μια προσαρμοσμένη εφαρμογή SAML
   - Χρησιμοποιήστε τις πληροφορίες SP που παρέχονται από το FastComments

2. **Διαμόρφωση Attributes**
   - Email: `user.mail` ή `user.userprincipalname`
   - First Name: `user.givenname`
   - Last Name: `user.surname`
   - Roles: `user.assignedroles` ή ομάδες καταλόγου

#### Okta

1. **Δημιουργία SAML Εφαρμογής**
   - Χρησιμοποιήστε "Create New App" και επιλέξτε SAML 2.0
   - Διαμορφώστε με τις πληροφορίες SP του FastComments

2. **Attribute Statements**
   - Email: `user.email`
   - FirstName: `user.firstName`
   - LastName: `user.lastName`
   - Roles: `user.groups` ή προσαρμοσμένα attributes

#### Google Workspace

1. **Προσθήκη SAML Εφαρμογής**
   - Μεταβείτε σε Apps > Web and mobile apps > Add App > Add custom SAML app
   - Διαμορφώστε με τις πληροφορίες SP του FastComments

2. **Αντιστοίχιση Attributes**
   - Email: Κύρια διεύθυνση email
   - First Name: Όνομα
   - Last Name: Επώνυμο
   - Roles: Ομάδες ή προσαρμοσμένα attributes

#### Active Directory Federation Services (ADFS)

1. **Προσθήκη Relying Party Trust**
   - Χρησιμοποιήστε το SP metadata URL του FastComments ή χειροκίνητη διαμόρφωση
   - Διαμορφώστε τις πληροφορίες SP όπως παρέχονται

2. **Claim Rules**
   - Email: Claim διεύθυνσης email
   - Name: Name ID claim
   - Roles: Συνδρομή σε ομάδες ή προσαρμοσμένα claims

### Ευελιξία Ονομάτων Attributes

Το FastComments δέχεται πληροφορίες ρόλων από πολλαπλά ονόματα attributes για να καλύψει διαφορετικές διαμορφώσεις IdP:

- `roles`
- `groups`
- `memberOf`
- `role`
- `group`
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

Αυτή η ευελιξία εξασφαλίζει συμβατότητα με διάφορους παρόχους ταυτότητας χωρίς να απαιτούνται συγκεκριμένες ονομασίες attributes.

### Δοκιμή της Διαμόρφωσής Σας

Μετά τη διαμόρφωση του παρόχου ταυτότητας σας:

1. Αποθηκεύστε τη διαμόρφωση του IdP
2. Δοκιμάστε με έναν αφιερωμένο test χρήστη
3. Επαληθεύστε ότι τα attributes αποστέλλονται σωστά
4. Ελέγξτε ότι οι ρόλοι αντιστοιχίζονται σωστά
5. Βεβαιωθείτε ότι η ροή πιστοποίησης ολοκληρώνεται επιτυχώς

Οι περισσότεροι πάροχοι ταυτότητας προσφέρουν εργαλεία δοκιμών SAML για να επικυρώσετε τη διαμόρφωση πριν την ανάπτυξη σε παραγωγικούς χρήστες.