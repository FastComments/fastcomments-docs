---
Από προεπιλογή, το FastComments επιτρέπει στους χρήστες να επεξεργάζονται τα σχόλιά τους.

Ωστόσο, είναι δυνατό να αποτραπεί αυτό.

Στη σελίδα προσαρμογής του widget, δείτε την επιλογή "Απενεργοποίηση επεξεργασίας".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-edit'; selector = '.disable-commenter-comment-edit'; title='Disable Comment Editing' app-screenshot-end]

- Αυτό επηρεάζει μόνο τους κανονικούς Commenters και όχι τους moderators ή admins, οι οποίοι θα εξακολουθούν να μπορούν να επεξεργάζονται.
- Αυτό θα επηρεάσει επίσης τις ενσωματώσεις API όταν περνάται το `contextUserId`. 

---