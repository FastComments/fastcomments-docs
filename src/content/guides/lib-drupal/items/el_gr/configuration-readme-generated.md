Πλοηγηθείτε στο **Διαχείριση > Διαμόρφωση > Περιεχόμενο > FastComments** (`/admin/config/content/fastcomments`).

### Ρυθμίσεις

- **Tenant ID** (απαραίτητο) - Το Tenant ID του FastComments. Βρείτε το κάτω από [Ρυθμίσεις > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Απαραίτητο για Ασφαλές SSO, επαλήθευση webhook και συγχρονισμό σελίδας. Βρίσκεται κάτω από [Ρυθμίσεις > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **SSO Mode** - Ενσωμάτωση Single Sign-On:
  - **Καμία** - Δεν υπάρχει SSO, οι χρήστες σχολιάζουν ως επισκέπτες ή δημιουργούν λογαριασμούς FastComments.
  - **Απλή** - Μεταβιβάζει πληροφορίες χρήστη Drupal (όνομα, email, avatar) στο FastComments χωρίς επαλήθευση στην πλευρά του server.
  - **Ασφαλής** - Χρησιμοποιεί επαλήθευση HMAC-SHA256 για να πιστοποιεί με ασφάλεια τους χρήστες Drupal με το FastComments (συνιστάται).
- **Commenting Style** - Ο τύπος του widget που θα εμφανιστεί:
  - **Ζωντανά Σχόλια** - Σχόλια με νήματα σε πραγματικό χρόνο.
  - **Streaming Chat** - Διεπαφή ζωντανού chat.
  - **Collab Chat** - Συνεργατική επισήμανση επιλογής κειμένου στην κύρια περιοχή περιεχομένου.
  - **Collab Chat + Comments** - Συνεργατικό chat και τυπικά σχόλια.
- **CDN URL** - Το URL του FastComments CDN (προεπιλογή: `https://cdn.fastcomments.com`).
- **Site URL** - Το URL του site FastComments (προεπιλογή: `https://fastcomments.com`).
- **Email notifications** - Αποστολή email στους συγγραφείς περιεχομένου όταν προστίθεται ένα νέο σχόλιο στο περιεχόμενό τους.

### Προσθήκη σχολίων σε τύπους περιεχομένου

Προσθέστε το πεδίο **FastComments** στους τύπους περιεχομένου σας μέσω **Δομή > Τύποι περιεχομένου > [type] > Διαχείριση πεδίων**. Το πεδίο διαθέτει διακόπτη κατάστασης και προαιρετικό προσαρμοσμένο αναγνωριστικό ανά οντότητα.

### Φιλοξενία δεδομένων στην ΕΕ

Για φιλοξενία δεδομένων στην ΕΕ, ενημερώστε:
- **CDN URL** σε `https://cdn-eu.fastcomments.com`
- **Site URL** σε `https://eu.fastcomments.com`