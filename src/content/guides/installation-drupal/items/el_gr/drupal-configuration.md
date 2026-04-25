Όλες οι ρυθμίσεις βρίσκονται κάτω από `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## Απαραίτητα

- **Tenant ID** - Το Tenant ID του FastComments σας. Find this under [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Απαιτείται για Secure SSO, επαλήθευση webhook και συγχρονισμό σελίδας. Found under [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).

## Commenting Style

Επιλέξτε το widget που ταιριάζει με τον τρόπο που θέλετε να συνομιλούν οι χρήστες στον ιστότοπό σας.

- **Live Comments** - Σχόλια με νήματα σε πραγματικό χρόνο.
- **Streaming Chat** - Διεπαφή ζωντανής συνομιλίας, κατάλληλη για εκδηλώσεις και livestreams.
- **Collab Chat** - Σχολιασμός με επιλογή κειμένου στην κύρια περιοχή περιεχομένου. Οι επισκέπτες επισημαίνουν κείμενο και ξεκινούν μια συζήτηση στο πλαίσιο.
- **Collab Chat + Comments** - Και collaborative chat και τυπικά σχόλια στην ίδια σελίδα.

## SSO Mode

- **None** - Χωρίς SSO. Οι χρήστες σχολιάζουν ως επισκέπτες ή δημιουργούν έναν λογαριασμό FastComments.
- **Simple** - Μεταβιβάζει πληροφορίες χρήστη του Drupal (όνομα, email, avatar) στο FastComments χωρίς επαλήθευση από τον διακομιστή.
- **Secure** - Χρησιμοποιεί HMAC-SHA256 για να επαληθεύει χρήστες Drupal με το FastComments. Συνιστάται όταν έχετε διαμορφώσει ένα API Secret.

Δείτε την ενότητα `Single Sign-On (SSO)` για λεπτομέρειες.

## Άλλες Ρυθμίσεις

- **CDN URL** - Προεπιλογή: `https://cdn.fastcomments.com`.
- **Site URL** - Προεπιλογή: `https://fastcomments.com`.
- **Email notifications** - Αποστολή email στον συγγραφέα του περιεχομένου όταν δημοσιεύεται νέο σχόλιο στο περιεχόμενό του.

Για την κατοικία δεδομένων στην ΕΕ, δείτε την ενότητα `EU Data Residency`.

---