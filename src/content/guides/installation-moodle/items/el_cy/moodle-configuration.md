Η σελίδα ρυθμίσεων του πρόσθετου βρίσκεται στο **Site Administration > Plugins > Local plugins > FastComments**. Οι διαθέσιμες επιλογές είναι:

#### Tenant ID

Το Tenant ID του FastComments σας. Βρείτε το στο <a href="https://fastcomments.com/auth/my-account" target="_blank">ταμπλό του FastComments</a> κάτω από τις ρυθμίσεις του λογαριασμού σας.

#### API Secret

Το API Secret κλειδί σας, απαιτείται για το Secure SSO mode. Βρείτε το στο <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">My Account > API Secret</a>.

#### SSO Mode

Επιλέξτε πώς γίνεται η πιστοποίηση των χρηστών. Δείτε την ενότητα [SSO Modes](#items-moodle-sso-modes) για λεπτομέρειες σχετικά με κάθε επιλογή.

- **Secure** (recommended) - πιστοποίηση υπογεγραμμένη με HMAC-SHA256 από την πλευρά του διακομιστή
- **Simple** - δεδομένα χρήστη στην πλευρά του πελάτη χωρίς υπογραφή
- **None** - ανώνυμα σχόλια, χωρίς ενσωμάτωση σύνδεσης Moodle

#### Page Contexts

Ελέγχει πού εμφανίζονται τα σχόλια:

- **Course pages** - σχόλια στις κύριες σελίδες του μαθήματος
- **Module/activity pages** - σχόλια σε μεμονωμένες δραστηριότητες και πόρους
- **Both** - σχόλια σε όλους τους τύπους σελίδων

#### Commenting Style

Επιλέξτε την εμπειρία σχολιασμού. Δείτε τα [Commenting Styles](#items-moodle-commenting-styles) για στιγμιότυπα οθόνης κάθε λειτουργίας.

- **Comments** - τυπικό νηματικό widget σχολίων κάτω από το περιεχόμενο της σελίδας
- **Collab Chat** - inline συζητήσεις επιλογής κειμένου με ενδείξεις παρουσίας
- **Both** - ενεργά και σχόλια και collab chat ταυτόχρονα

#### CDN URL

Το URL του FastComments CDN. Προεπιλογή είναι `https://cdn.fastcomments.com`. Αλλάξτε το στο EU CDN URL εάν τα δεδομένα σας φιλοξενούνται στην περιοχή της ΕΕ.