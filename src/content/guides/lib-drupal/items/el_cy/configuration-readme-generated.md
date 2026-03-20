Πλοηγηθείτε στο **Διαχείριση > Διαμόρφωση > Περιεχόμενο > FastComments** (`/admin/config/content/fastcomments`).

### Ρυθμίσεις

- **Tenant ID** (απαιτείται) - Το Tenant ID σας στο FastComments. Βρείτε το κάτω από [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Απαραίτητο για Secure SSO, επαλήθευση webhook και συγχρονισμό σελίδων. Βρίσκεται κάτω από [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **SSO Mode** - Ενσωμάτωση Single Sign-On:
  - **None** - Χωρίς SSO, οι χρήστες σχολιάζουν ως επισκέπτες ή δημιουργούν λογαριασμούς FastComments.
  - **Simple** - Μετάδοση πληροφοριών χρήστη Drupal (όνομα, email, avatar) στο FastComments χωρίς επαλήθευση από τον διακομιστή.
  - **Secure** - Χρήση HMAC-SHA256 επαλήθευσης για ασφαλή πιστοποίηση χρηστών Drupal με το FastComments (συνιστάται).
- **Commenting Style** - Ο τύπος widget που θα εμφανίζεται:
  - **Live Comments** - Σχόλια σε πραγματικό χρόνο με νήματα.
  - **Streaming Chat** - Διεπαφή ζωντανής συνομιλίας.
  - **Collab Chat** - Συνεργατική επισήμανση επιλογής κειμένου στην κύρια περιοχή περιεχομένου.
  - **Collab Chat + Comments** - Συνεργατική συνομιλία και κανονικά σχόλια.
- **CDN URL** - FastComments CDN URL (προεπιλογή: `https://cdn.fastcomments.com`).
- **Site URL** - FastComments site URL (προεπιλογή: `https://fastcomments.com`).
- **Email notifications** - Αποστολή email στον/στην συγγραφέα του περιεχομένου όταν δημοσιεύεται νέο σχόλιο στο περιεχόμενό τους.

### Προσθήκη σχολίων σε τύπους περιεχομένου

Προσθέστε το πεδίο **FastComments** στους τύπους περιεχομένου σας μέσω **Δομή > Τύποι περιεχομένου > [type] > Διαχείριση πεδίων**. Το πεδίο διαθέτει διακόπτη κατάστασης και προαιρετικό προσαρμοσμένο αναγνωριστικό ανά οντότητα.

### Φιλοξενία δεδομένων στην ΕΕ

Για φιλοξενία δεδομένων στην ΕΕ, ενημερώστε:
- **CDN URL** σε `https://cdn-eu.fastcomments.com`
- **Site URL** σε `https://eu.fastcomments.com`