Όλες οι ρυθμίσεις βρίσκονται στο `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## Απαιτείται

- **Tenant ID** - Το FastComments Tenant ID σας. Βρείτε το στο [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Απαιτείται για το Secure SSO, την επαλήθευση webhook και τον συγχρονισμό σελίδων. Βρίσκεται στο [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).

## Commenting Style

Επιλέξτε το widget που ταιριάζει με τον τρόπο που θέλετε να συνομιλούν οι χρήστες στον ιστότοπό σας.

- **Live Comments** - Σχόλια σε πραγματικό χρόνο με νήματα.
- **Streaming Chat** - Διεπαφή ζωντανού chat, κατάλληλη για εκδηλώσεις και ζωντανές μεταδόσεις.
- **Collab Chat** - Σχολιασμός με επιλογή κειμένου στην κύρια περιοχή περιεχομένου. Οι επισκέπτες επισημαίνουν κείμενο και ξεκινούν μια συζήτηση στο πλαίσιο του κειμένου.
- **Collab Chat + Comments** - Ταυτόχρονα collab chat και κανονικά σχόλια στην ίδια σελίδα.

## SSO Mode

- **None** - Χωρίς SSO. Οι χρήστες σχολιάζουν ως επισκέπτες ή δημιουργούν λογαριασμό FastComments.
- **Simple** - Μεταφέρει πληροφορίες χρήστη Drupal (name, email, avatar) στο FastComments χωρίς επαλήθευση από τον διακομιστή.
- **Secure** - Χρησιμοποιεί HMAC-SHA256 για την επαλήθευση χρηστών Drupal με το FastComments. Συνιστάται όταν έχετε ρυθμίσει API Secret.

Δείτε την ενότητα `Single Sign-On (SSO)` για λεπτομέρειες.

## Άλλες Ρυθμίσεις

- **CDN URL** - Προεπιλογή: `https://cdn.fastcomments.com`.
- **Site URL** - Προεπιλογή: `https://fastcomments.com`.
- **Email notifications** - Αποστολή email στον δημιουργό περιεχομένου όταν δημοσιεύεται νέο σχόλιο στο περιεχόμενό τους.

Για τη διαμονή δεδομένων στην ΕΕ, δείτε την ενότητα `EU Data Residency`.