---
Από προεπιλογή, το FastComments θα εμφανίζει το όνομα του χρήστη όπως το εισήγαγε, ή όπως μας μεταφέρθηκε μέσω SSO.

Ωστόσο, μπορεί να είναι επιθυμητό να αποκρύψετε ή να εμφανίσετε το όνομα του χρήστη με διαφορετικό τρόπο. Για παράδειγμα, αν το όνομα του χρήστη είναι Allen Rex, ίσως
θέλετε να εμφανίζεται μόνο "Allen R.".

Αυτό μπορεί να γίνει χωρίς κώδικα στο UI προσαρμογής του Widget, στην ρύθμιση με την ονομασία `Commenter Name Format`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; title='Change Name Format' app-screenshot-end]

Οι διαθέσιμες μορφές είναι:

- Capitalize (display example user as Example User)
- Last Initial (display Example User as Example U.)
- All Initials (display Example User as E. U.)
- Show "Anonymous"

Η επίδραση αυτής της αλλαγής είναι άμεση. Οι χρήστες θα βλέπουν ακόμα το πλήρες όνομα χρήστη τους στην κορυφή της περιοχής σχολίων, για τους ίδιους, αλλά τα σχόλιά τους θα εμφανίζουν
το τροποποιημένο όνομα χρήστη.

Τα ονόματα χρήστη μάσκονται από την πλευρά του διακομιστή για την προστασία των χρηστών.

---