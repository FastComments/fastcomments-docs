The Recent Comments widget displays the most recent comments posted across your entire site. Είναι χρήσιμο σε πλαϊνές μπάρες, υποσέλιδα ή οπουδήποτε θέλετε να προβάλετε πρόσφατη δραστηριότητα για να ενθαρρύνετε περαιτέρω ανάγνωση.

## Επιλογές

- **Τίτλος** (προαιρετικό): Ο τίτλος που εμφανίζεται πάνω από τη λίστα. Προεπιλογή: "Recent Comments".
- **Πλήθος** (προαιρετικό): Πόσα σχόλια να εμφανίζονται. Εύρος 1 έως 50. Προεπιλογή 5.

## How to Add It

### Inside a Post or Page

In the block editor, add a **Shortcode** block and paste:

[inline-code-attrs-start title = 'Σύντομος κώδικας Recent Comments'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_comments count="5"]
[inline-code-end]

The `count` attribute accepts any value between 1 and 50.

### In a Sidebar or Footer (Classic Themes)

Go to **Appearance > Widgets** in your WordPress admin. From the block inserter, search for "FastComments" and choose **FastComments: Recent Comments**. Σύρετέ το σε μια πλαϊνή στήλη, κεφαλίδα ή περιοχή υποσέλιδου, και στη συνέχεια διαμορφώστε τον τίτλο και το πλήθος από τον πίνακα του widget.

### In a Block Theme (Full Site Editing)

Open the **Site Editor** under **Appearance > Editor**. Navigate to the template part where the widget should appear, insert a **Legacy Widget** block, and select **FastComments: Recent Comments** from the dropdown.

## Troubleshooting

The widget only renders after FastComments setup is complete and a tenant ID is stored. Εάν η περιοχή του widget είναι κενή, ολοκληρώστε τη ρύθμιση στο **FastComments** στο διαχειριστικό του WordPress και ανανεώστε τη σελίδα.