---
[related-parameter-start name = 'disableImageRedirect'; type = 'boolean'; related-parameter-end]

Από προεπιλογή, το FastComments επιτρέπει στους χρήστες να ανεβάζουν εικόνες. Όταν ένας χρήστης κάνει κλικ σε αυτήν την εικόνα, το FastComments θα, εξ ορισμού,
ανοίξει μια νέα καρτέλα για να δείξει την εικόνα σε πλήρες μέγεθος. Ορισμός αυτής της σημαίας σε true απενεργοποιεί αυτή τη συμπεριφορά:

[code-example-start config = {disableImageRedirect: true}; linesToHighlight = [6]; title = 'Disable Image Redirect'; code-example-end]

Εάν δεν σκοπεύετε να χειριστείτε το κλικ στην εικόνα εσείς οι ίδιοι (δείτε [onImageClicked](#callbacks)), προτείνουμε να συνδυάσετε αυτό με κάποια μορφοποίηση
ώστε να αφαιρέσετε την εντύπωση ότι η εικόνα μπορεί να κλικάρεται.

---