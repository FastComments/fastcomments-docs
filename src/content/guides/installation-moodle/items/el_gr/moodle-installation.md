#### Λήψη του πρόσθετου

Κατεβάστε το πιο πρόσφατο αρχείο ZIP από το <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">αποθετήριο FastComments Moodle στο GitHub</a>.

#### Αποσυμπίεση στον κατάλογο του Moodle σας

Αποσυμπιέστε το αρχείο ZIP στην εγκατάσταση Moodle σας ώστε το πρόσθετο να βρίσκεται στη διαδρομή `<moodle-root>/local/fastcomments`. Ο φάκελος του πρόσθετου θα πρέπει να περιέχει απευθείας τα `version.php`, `lib.php` και άλλα αρχεία του πρόσθετου (όχι μέσα σε υποφάκελο).

Για παράδειγμα:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Εγκατάσταση μέσω του διαχειριστή του Moodle

Συνδεθείτε ως διαχειριστής του ιστότοπου και μεταβείτε στο **Διαχείριση ιστότοπου > Ειδοποιήσεις**. Το Moodle θα εντοπίσει το νέο πρόσθετο και θα σας ζητήσει να εκτελέσετε την εγκατάσταση.

#### Διαμόρφωση του πρόσθετου

Μετά την εγκατάσταση, μεταβείτε στο **Διαχείριση ιστότοπου > Πρόσθετα > Τοπικά πρόσθετα > FastComments** για να εισαγάγετε τις ρυθμίσεις σας. Δείτε την ενότητα [Διαμόρφωση](#items-moodle-configuration) για λεπτομέρειες σχετικά με κάθε επιλογή.

---