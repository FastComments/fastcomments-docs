The Recent Discussions widget displays the pages on your site with the most recent comment activity. It's useful for highlighting threads that are still being added to, so visitors can jump back into active conversations rather than landing on quiet pages.

## Options

- **Title** (optional): Ο τίτλος που εμφανίζεται πάνω από τη λίστα. Προεπιλογή είναι "Πρόσφατες Συζητήσεις".
- **Count** (optional): Πόσες συζητήσεις να εμφανιστούν. Εύρος 1 έως 50. Προεπιλογή είναι 20.

## How to Add It

### Inside a Post or Page

Στον επεξεργαστή μπλοκ, προσθέστε ένα μπλοκ **Shortcode** και επικολλήστε:

[inline-code-attrs-start title = 'Σύντομος κώδικας πρόσφατων συζητήσεων'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_discussions count="20"]
[inline-code-end]

Το χαρακτηριστικό `count` δέχεται οποιαδήποτε τιμή μεταξύ 1 και 50.

### In a Sidebar or Footer (Classic Themes)

Μεταβείτε στο **Appearance > Widgets** στη διαχείριση του WordPress. Από τον εισαγωγέα μπλοκ, αναζητήστε το "FastComments" και επιλέξτε **FastComments: Recent Discussions**. Σύρετε το σε μια πλευρική στήλη, κεφαλίδα ή περιοχή υποσέλιδου, στη συνέχεια ρυθμίστε τον τίτλο και την καταμέτρηση από τον πίνακα του widget.

### In a Block Theme (Full Site Editing)

Ανοίξτε τον **Site Editor** μέσω του **Appearance > Editor**. Περιηγηθείτε στο τμήμα προτύπου όπου πρέπει να εμφανιστεί το widget, εισάγετε ένα μπλοκ **Legacy Widget** και επιλέξτε **FastComments: Recent Discussions** από το αναπτυσσόμενο μενού.

## Troubleshooting

Το widget εμφανίζεται μόνο μετά την ολοκλήρωση της ρύθμισης του FastComments και την αποθήκευση ενός tenant ID. Εάν η περιοχή του widget είναι κενή, ολοκληρώστε τη ρύθμιση μέσω του **FastComments** στη διαχείριση του WordPress και φορτώστε ξανά τη σελίδα.

Εάν η σειρά των συζητήσεων φαίνεται παλιά, ελέγξτε ότι οι υποκείμενες σελίδες έχουν ολοκληρώσει το συγχρονισμό στον πίνακα ελέγχου του FastComments. Το widget διαβάζει ζωντανά δεδομένα, έτσι τα πρόσφατα εισαχθέντα σχόλια μπορεί να χρειαστούν λίγο χρόνο για να εμφανιστούν.