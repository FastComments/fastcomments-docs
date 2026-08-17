---
Για να περάσετε τον συνδεδεμένο χρήστη στο widget αυτόματα, οι ετικέτες διαβάζουν τον τρέχοντα χρήστη από το αίτημα. Βεβαιωθείτε ότι το έργο σας περιλαμβάνει και τα δύο (είναι ενεργοποιημένα εξ ορισμού σε ένα τυπικό έργο Django):

- `django.template.context_processors.request` in `TEMPLATES["OPTIONS"]["context_processors"]`
- `django.contrib.auth.middleware.AuthenticationMiddleware` in `MIDDLEWARE`

Χωρίς ένα αίτημα στο πλαίσιο του προτύπου, τα widgets αποδίδονται για έναν ανώνυμο επισκέπτη. Μπορείτε πάντα να περάσετε έναν χρήστη ρητά: `{% fastcomments user=some_user %}`.
---