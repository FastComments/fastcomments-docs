Το widget Recent Comments εμφανίζει τα πιο πρόσφατα σχόλια που έχουν δημοσιευτεί σε ολόκληρο τον ιστότοπό σας. Είναι χρήσιμο σε sidebars, footers ή οπουδήποτε θέλετε να αναδείξετε πρόσφατη δραστηριότητα για να ενθαρρύνετε περαιτέρω ανάγνωση.

## Επιλογές

- **Title** (προαιρετικό): Η επικεφαλίδα που εμφανίζεται πάνω από τη λίστα. Προεπιλογή: "Recent Comments".
- **Count** (προαιρετικό): Πόσα σχόλια θα εμφανίζονται. Εύρος 1 έως 50. Προεπιλογή: 5.

## Πώς να το Προσθέσετε

### Μέσα σε Άρθρο ή Σελίδα

Στον επεξεργαστή block, προσθέστε ένα block **Shortcode** και επικολλήστε:

[inline-code-attrs-start title = 'Shortcode Recent Comments'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_comments count="5"]
[inline-code-end]

Το attribute `count` δέχεται οποιαδήποτε τιμή μεταξύ 1 και 50.

### Σε Sidebar ή Footer (Κλασικά Θέματα)

Μεταβείτε στο **Appearance > Widgets** στη διαχείριση του WordPress. Από τον block inserter, αναζητήστε "FastComments" και επιλέξτε **FastComments: Recent Comments**. Σύρετέ το σε μια περιοχή sidebar, header ή footer και, στη συνέχεια, ρυθμίστε τον τίτλο και το count από τον πίνακα του widget.

### Σε Block Theme (Full Site Editing)

Ανοίξτε τον **Site Editor** κάτω από **Appearance > Editor**. Μεταβείτε στο template part όπου πρέπει να εμφανιστεί το widget, εισαγάγετε ένα block **Legacy Widget** και επιλέξτε **FastComments: Recent Comments** από το dropdown.

## Αντιμετώπιση Προβλημάτων

Το widget εμφανίζεται μόνο αφού ολοκληρωθεί η ρύθμιση του FastComments και έχει αποθηκευτεί ένα tenant ID. Αν η περιοχή του widget είναι κενή, ολοκληρώστε τη ρύθμιση κάτω από **FastComments** στη διαχείριση του WordPress και φορτώστε ξανά τη σελίδα.