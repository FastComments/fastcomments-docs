#### Λήψη του πρόσθετου

Κατεβάστε το πιο πρόσφατο αρχείο ZIP από το <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">Αποθετήριο FastComments Moodle στο GitHub</a>.

#### Αποσυμπίεση στον κατάλογο Moodle σας

Αποσυμπιέστε το ZIP στην εγκατάσταση του Moodle ώστε το πρόσθετο να βρίσκεται στο `<moodle-root>/local/fastcomments`. Ο φάκελος του πρόσθετου θα πρέπει να περιέχει τα `version.php`, `lib.php` και άλλα αρχεία του πρόσθετου απευθείας (όχι σε υποφάκελο).

Για παράδειγμα:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Εγκατάσταση μέσω του διαχειριστή του Moodle

Συνδεθείτε ως διαχειριστής του ιστότοπου και μεταβείτε στο **Διαχείριση ιστότοπου > Ειδοποιήσεις**. Το Moodle θα εντοπίσει το νέο πρόσθετο και θα σας ζητήσει να εκτελέσετε την εγκατάσταση.

#### Διαμόρφωση του πρόσθετου

Μετά την εγκατάσταση, μεταβείτε στο **Διαχείριση ιστότοπου > Πρόσθετα > Τοπικά πρόσθετα > FastComments** για να εισαγάγετε τις ρυθμίσεις σας. Δείτε την [Διαμόρφωση](#moodle-configuration) ενότητα για λεπτομέρειες σχετικά με κάθε επιλογή.

---