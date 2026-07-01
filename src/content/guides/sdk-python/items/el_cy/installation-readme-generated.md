### Εγκατάσταση από το GitHub

Εγκατάσταση άμεσα από μια ετικέτα έκδοσης (συνιστάται, πλήρως επαναλήψιμη):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

Καρφιτρώστε την ετικέτα αντί για ένα κλάδο ώστε οι κατασκευές να είναι ντετερμινιστικές. Η ίδια μορφή λειτουργεί στο `requirements.txt`:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

Κάθε ετικετοποιημένη [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) διαθέτει επίσης ένα κατασκευασμένο wheel συνημμένο εάν προτιμάτε να εγκαταστήσετε ένα δυαδικό τεχνητό απευθείας.

### Περιεχόμενα Βιβλιοθήκης

Αυτή η βιβλιοθήκη περιέχει δύο modules: τον δημιουργημένο πελάτη API και τη βασική βιβλιοθήκη Python που περιλαμβάνει χειροκίνητες βοηθητικές λειτουργίες για να διευκολύνει τη χρήση του API, συμπεριλαμβανομένης της υποστήριξης SSO.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Δημόσια vs Ασφαλείς API

Για τον πελάτη API, υπάρχουν τρεις κλάσεις, `DefaultApi`, `PublicApi` και `ModerationApi`. Η `DefaultApi` περιέχει μεθόδους που απαιτούν το κλειδί API σας, και η `PublicApi` περιέχει μεθόδους που μπορούν να κληθούν απευθείας από έναν περιηγητή/συσκευή κινητής κ.λπ. χωρίς έλεγχο ταυτότητας. Η `ModerationApi` παρέχει μια εκτενή σειρά από ζωντανά και γρήγορα API διαmoderation. Κάθε μέθοδος της `ModerationApi` δέχεται μια παράμετρο `sso` και μπορεί να πιστοποιηθεί μέσω SSO ή ενός cookie συνεδρίας FastComments.com.