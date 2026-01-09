[related-parameter-start name = 'disableImageRedirect'; type = 'boolean'; related-parameter-end]

Από προεπιλογή, το FastComments επιτρέπει στους χρήστες να ανεβάζουν εικόνες. Όταν ο χρήστης κάνει κλικ σε αυτήν την εικόνα, το FastComments, από προεπιλογή, θα ανοίξει μια νέα καρτέλα για να εμφανίσει την εικόνα σε πλήρες μέγεθος. Η ρύθμιση αυτής της σημαίας σε true απενεργοποιεί αυτή τη συμπεριφορά:

[code-example-start config = {disableImageRedirect: true}; linesToHighlight = [6]; title = 'Disable Image Redirect'; code-example-end]

Αν δεν σκοπεύετε να χειριστείτε εσείς το κλικ στην εικόνα (βλέπε [onImageClicked](#callbacks)), συνιστούμε να το συνδυάσετε με κάποια στυλ ώστε να αφαιρέσετε την εντύπωση ότι η εικόνα μπορεί να γίνει κλικ.

---