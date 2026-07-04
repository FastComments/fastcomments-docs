Για να περάσετε τον συνδεδεμένο χρήστη στο widget αυτόματα, οι ετικέτες διαβάζουν τον τρέχοντα χρήστη από το αίτημα. Βεβαιωθείτε ότι το έργο σας περιλαμβάνει και τα δύο (είναι ενεργά εξ 'ορισμού σε ένα τυπικό έργο Django):

- `django.template.context_processors.request` στο `TEMPLATES["OPTIONS"]["context_processors"]`
- `django.contrib.auth.middleware.AuthenticationMiddleware` στο `MIDDLEWARE`

Χωρίς αίτημα στο context του προτύπου, τα widgets εμφανίζονται για ανώνυμο επισκέπτη. Μπορείτε πάντα να περάσετε έναν χρήστη ρητά: `{% fastcomments user=some_user %}`.