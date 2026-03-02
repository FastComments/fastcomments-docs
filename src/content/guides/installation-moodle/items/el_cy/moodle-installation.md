#### Λήψη του πρόσθετου

Κατεβάστε το πιο πρόσφατο ZIP release από το <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">αποθετήριο FastComments Moodle στο GitHub</a>.

#### Αποσυμπίεση στον κατάλογο του Moodle σας

Αποσυμπιέστε το ZIP στην εγκατάσταση του Moodle έτσι ώστε το πρόσθετο να βρίσκεται στο `<moodle-root>/local/fastcomments`. Ο κατάλογος του πρόσθετου πρέπει να περιέχει απευθείας `version.php`, `lib.php`, και άλλα αρχεία του πρόσθετου (όχι σε υποφάκελο).

For example:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Εγκατάσταση μέσω του διαχειριστή του Moodle

Συνδεθείτε ως διαχειριστής του ιστότοπου και μεταβείτε σε **Διαχείριση ιστότοπου > Ειδοποιήσεις**. Το Moodle θα ανιχνεύσει το νέο πρόσθετο και θα σας ζητήσει να εκτελέσετε την εγκατάσταση.

#### Ρύθμιση του πρόσθετου

Μετά την εγκατάσταση, πηγαίνετε σε **Διαχείριση ιστότοπου > Πρόσθετα > Τοπικά πρόσθετα > FastComments** για να εισάγετε τις ρυθμίσεις σας. Δείτε την ενότητα [Διαμόρφωση](#moodle-configuration) για λεπτομέρειες σχετικά με κάθε επιλογή.