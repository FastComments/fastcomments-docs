### Εγκατάσταση από το GitHub

Εγκαταστήστε απευθείας από μια ετικέτα έκδοσης (συνιστάται, πλήρως αναπαραγώγιμο):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Καρφιτρώστε την ετικέτα αντί για ένα κλαδί ώστε οι κατασκευές να είναι καθοριστικές. Η ίδια μορφή λειτουργεί στο `requirements.txt`:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Κάθε ετικετοποιημένη [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) έχει επίσης ένα προ-δημιουργημένο wheel συνημμένο εάν προτιμάτε να εγκαταστήσετε ένα δυαδικό τεχνητό αντικείμενο απευθείας.

### Περιεχόμενα Βιβλιοθήκης

Αυτή η βιβλιοθήκη περιέχει δύο modules: τον παραγόμενο πελάτη API και τη βασική βιβλιοθήκη Python που περιέχει χειρογράφως γραμμένα βοηθητικά εργαλεία για να διευκολύνει τη χρήση του API, συμπεριλαμβανομένης της υποστήριξης SSO.

- [Τεκμηρίωση Βιβλιοθήκης Πελάτη API](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Τεκμηρίωση Βασικής Βιβλιοθήκης, Συμπεριλαμβανομένων Παραδειγμάτων SSO](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Δημόσια vs Ασφαλισμένα APIs

Για τον πελάτη API, υπάρχουν τρεις κλάσεις, `DefaultApi`, `PublicApi` και `ModerationApi`. Η `DefaultApi` περιέχει μεθόδους που απαιτούν το κλειδί API σας, και η `PublicApi` περιέχει μεθόδους που μπορούν να κληθούν απευθείας από έναν περιηγητή/συσκευή κινητής κίνησης κ.λπ. χωρίς έλεγχο ταυτότητας. Η `ModerationApi` παρέχει μια εκτενή σειρά ζωντανών και γρήγορων APIs διαχείρι

ασης. Κάθε μέθοδος της `ModerationApi` δέχεται μια παράμετρο `sso` και μπορεί να πιστοποιηθεί μέσω SSO ή ενός cookie συνεδρίας FastComments.com.