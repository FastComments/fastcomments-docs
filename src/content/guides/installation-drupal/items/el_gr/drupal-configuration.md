All settings live under `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## Απαιτείται

- **Tenant ID** - Το FastComments Tenant ID σας. Βρείτε το κάτω από [Ρυθμίσεις > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Απαιτείται για ασφαλή SSO, επαλήθευση webhook και συγχρονισμό σελίδας. Βρίσκεται κάτω από [Ρυθμίσεις > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).

## Αποθήκευση του API Secret

Το module διαβάζει το API Secret από την πρώτη από τις παρακάτω πηγές που έχει τιμή:

1. **Key module** - Εγκαταστήστε το [Key](https://www.drupal.org/project/key), δημιουργήστε ένα κλειδί *Authentication* και επιλέξτε το κάτω από **API Secret key** στη φόρμα ρυθμίσεων. Το κλειδί μπορεί να υποστηρίζεται από μια μεταβλητή περιβάλλοντος ή ένα αρχείο εκτός του web root.
2. **settings.php** - Προσθέστε `$config['fastcomments.settings']['api_secret'] = 'your-secret';` στο `settings.php` ή `settings.local.php`. Το μυστικό δεν εισέρχεται ποτέ στην εξαγόμενη διαμόρφωση.
3. **The settings form** - Εισάγετε το στο πεδίο **API Secret**. Αυτό αποθηκεύει την τιμή στο `fastcomments.settings`, έτσι εμφανίζεται στις εξαγωγές διαμόρφωσης. Η αναφορά κατάστασης του Drupal προειδοποιεί όταν αυτή είναι η ενεργή πηγή.

Η φόρμα δείχνει ποια πηγή είναι ενεργή. Αφήνοντας το πεδίο κενό διατηρεί το αποθηκευμένο μυστικό. Επιλέξτε **Αφαιρέστε το API Secret που αποθηκεύεται στη διαμόρφωση** για να το διαγράψετε μετά τη μεταφορά του μυστικού στο Key module ή στο `settings.php`.

## Στυλ Σχολιασμού

Επιλέξτε το widget που ταιριάζει με το πώς θέλετε οι χρήστες να μιλούν στον ιστότοπό σας.

- **Live Comments** - Σχόλια σε πραγματικό χρόνο με νηματοποίηση.
- **Streaming Chat** - Διεπαφή ζωντανής συνομιλίας, κατάλληλη για εκδηλώσεις και ζωντανές μεταδόσεις.
- **Collab Chat** - Σχόλιο με επισήμανση κειμένου στην κύρια περιοχή περιεχομένου. Οι επισκέπτες επισημαίνουν κείμενο και ξεκινούν συζήτηση στο πλαίσιο.
- **Collab Chat + Comments** - Και το collab chat και τα τυπικά σχόλια στην ίδια σελίδα.

## Λειτουργία SSO

- **None** - Χωρίς SSO. Οι χρήστες σχολιάζουν ως επισκέπτες ή δημιουργούν λογαριασμό FastComments.
- **Simple** - Μεταβιβάζει πληροφορίες χρήστη Drupal (όνομα, email, avatar) στο FastComments χωρίς επαλήθευση από τον διακομιστή.
- **Secure** - Χρησιμοποιεί HMAC‑SHA256 για επαλήθευση χρηστών Drupal με FastComments. Συνιστάται όταν έχετε ρυθμίσει ένα API Secret.

Δείτε την ενότητα `Single Sign-On (SSO)` για λεπτομέρειες.

## Άλλες Ρυθμίσεις

- **CDN URL** - Προεπιλογή είναι `https://cdn.fastcomments.com`.
- **Site URL** - Προεπιλογή είναι `https://fastcomments.com`.
- **Email notifications** - Στέλνει email σε συγγραφέα περιεχομένου όταν δημοσιεύεται νέο σχόλιο στο περιεχόμενό του.

Για κατοικία δεδομένων EU, δείτε την ενότητα `EU Data Residency`.

---