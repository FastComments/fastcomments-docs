### Εγκατάσταση από το GitHub

Εγκαταστήστε απευθείας από ετικέτα έκδοσης (συνιστάται, πλήρως αναπαραγώγιμο):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

Σημειώστε την ετικέτα αντί για ένα κλάδο ώστε οι κατασκευές να είναι ντετερμινιστικές. Η ίδια μορφή λειτουργεί στο `requirements.txt`:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

Κάθε ετικετομένη [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) διαθέτει επίσης ένα προδημιουργημένο wheel συνημμένο εάν προτιμάτε να εγκαταστήσετε ένα δυαδικό artefact απευθείας.

### Περιεχόμενα Βιβλιοθήκης

Αυτή η βιβλιοθήκη περιλαμβάνει δύο modules: τον παραγόμενο πελάτη API και τη βασική βιβλιοθήκη Python που περιέχει χειρογραφούμενα βοηθητικά εργαλεία για να διευκολύνουν τη δουλειά με το API, συμπεριλαμβανομένης της υποστήριξης SSO.

- [Τεκμηρίωση Βιβλιοθήκης Πελάτη API](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Τεκμηρίωση Κεντρικής Βιβλιοθήκης, Συμπεριλαμβανομένων Παραδειγμάτων SSO](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Δημόσια vs Ασφαλισμένα APIs

Για τον πελάτη API, υπάρχουν τρεις κλάσεις, `DefaultApi`, `PublicApi` και `ModerationApi`. Η `DefaultApi` περιέχει μεθόδους που απαιτούν το κλειδί API σας, και η `PublicApi` περιέχει μεθόδους που μπορούν να κληθούν απευθείας από έναν περιηγητή/συσκευή κινητής/κτλ χωρίς έλεγχο ταυτότητας. Η `ModerationApi` προσφέρει μια εκτενή σειρά ζωντανών και γρήγορων APIs moderation. Κάθε μέθοδος της `ModerationApi` δέχεται μια παράμετρο `sso` και μπορεί να πιστοποιηθεί μέσω SSO ή μέσω cookie συνεδρίας FastComments.com.