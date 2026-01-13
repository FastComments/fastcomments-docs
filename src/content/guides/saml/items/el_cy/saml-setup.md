Η ρύθμιση του SAML authentication στο FastComments απαιτεί τόσο ρυθμίσεις στο admin dashboard σας όσο και ρύθμιση στον πάροχο ταυτότητας σας.

### Prerequisites

Πριν ρυθμίσετε το SAML, βεβαιωθείτε ότι έχετε:

- Ένα FastComments Flex ή Pro πλάνο (το SAML δεν είναι διαθέσιμο στο Creators πλάνο)
- Διαχειριστική πρόσβαση στον λογαριασμό FastComments σας
- Διαχειριστική πρόσβαση στον πάροχο ταυτότητας σας
- Τα μεταδεδομένα SAML ή πληροφορίες πιστοποιητικού του IdP σας

### Accessing SAML Configuration

1. Συνδεθείτε στο [FastComments admin dashboard](https://fastcomments.com/auth/my-account)
2. Πλοηγηθείτε στις **API/SSO Settings** στην αριστερή στήλη
3. Κάντε κλικ στο κουμπί **SAML Config**

Εάν δεν βλέπετε το κουμπί SAML Config, επαληθεύστε ότι:
- Ο λογαριασμός σας έχει το απαιτούμενο πακέτο (Flex ή Pro)
- Έχετε δικαιώματα διαχειριστή
- Ο χρήστης σας έχει ρόλους API Admin ή Admin Admin

### Basic SAML Configuration

#### Enable SAML Authentication

1. Επιλέξτε το checkbox **Enable SAML Authentication**
2. Αυτό ενεργοποιεί το SAML για τον tenant σας και καθιστά διαθέσιμα τα πεδία διαμόρφωσης

#### Required Fields

**IdP Single Sign-On URL** *(Required)*
- Το URL όπου οι χρήστες θα ανακατευθυνθούν για αυθεντικοποίηση
- Συνήθως παρέχεται από τον πάροχο ταυτότητας σας
- Παράδειγμα: `https://your-company.okta.com/app/fastcomments/sso/saml`

**IdP X.509 Certificate** *(Required)*
- Το δημόσιο πιστοποιητικό από τον πάροχο ταυτότητας σας
- Χρησιμοποιείται για την επαλήθευση της αυθεντικότητας των SAML responses
- Θα πρέπει να περιλαμβάνει ολόκληρο το πιστοποιητικό με τους δείκτες BEGIN/END
- Παράδειγμα μορφής:
```
-----BEGIN CERTIFICATE-----
MIICXjCCAcegAwIBAgIBADANBgkqhkiG9w0BAQsFADA...
-----END CERTIFICATE-----
```

#### Optional Fields

**IdP Entity ID / Issuer**
- Αναγνωρίζει τον πάροχο ταυτότητας σας
- Εάν αφεθεί κενό, προεπιλέγεται στο URL του FastComments σας
- Θα πρέπει να ταιριάζει με τον issuer που έχει ρυθμιστεί στον IdP σας

### Advanced Configuration

#### Security Settings

**Signature Algorithm**
- Προεπιλογή σε SHA-256 (συνιστώμενο)
- Επιλογές: SHA-1, SHA-256, SHA-512
- Θα πρέπει να ταιριάζει με τη ρύθμιση του IdP σας

**Digest Algorithm**
- Προεπιλογή σε SHA-256 (συνιστώμενο)
- Χρησιμοποιείται για τον υπολογισμό του digest στις SAML responses
- Θα πρέπει να ταιριάζει με τη ρύθμιση του IdP σας

**Name ID Format**
- Προεπιλογή σε μορφή Email Address
- Καθορίζει πώς μορφοποιούνται τα αναγνωριστικά χρηστών
- Συνήθεις επιλογές: Email Address, Persistent, Transient

#### Encryption (Optional)

**Private Key for Decryption**
- Απαιτείται μόνο εάν ο IdP σας κρυπτογραφεί assertions SAML
- Επικολλήστε το ιδιωτικό κλειδί που χρησιμοποιείται για αποκρυπτογράφηση
- Οι περισσότερες εγκαταστάσεις δεν απαιτούν κρυπτογράφηση assertions

### Saving Configuration

1. Επαληθεύστε όλες τις ρυθμίσεις για ακρίβεια
2. Κάντε κλικ στο **Save SAML Configuration**
3. Το σύστημα θα επικυρώσει τη ρύθμισή σας
4. Εάν επιτύχει, θα δείτε μήνυμα επιβεβαίωσης

### Next Steps

Μετά την αποθήκευση της SAML ρύθμισης του FastComments σας:

1. Ρυθμίστε τον πάροχο ταυτότητας χρησιμοποιώντας τις πληροφορίες του Service Provider
2. Δοκιμάστε τη ροή αυθεντικοποίησης
3. Ρυθμίστε ρόλους και δικαιώματα χρηστών όπως χρειάζεται

Οι πληροφορίες του Service Provider που απαιτούνται για τη ρύθμιση του IdP σας θα εμφανιστούν μόλις το SAML ενεργοποιηθεί.