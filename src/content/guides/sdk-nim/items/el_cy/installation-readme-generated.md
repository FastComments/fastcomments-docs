### Χρήση Nimble

```bash
nimble install fastcomments
```

### Δόμηση από την Πηγή

```bash
nimble build
```

### Περιεχόμενα Βιβλιοθήκης

Αυτή η βιβλιοθήκη περιέχει τον παραγόμενο πελάτη API και τα εργαλεία SSO για να διευκολύνει τη δουλειά με το API.

- [Τεκμηρίωση Βιβλιοθήκης Πελάτη API](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Δημόσια vs Προστατευμένα API

Για τον πελάτη API, υπάρχουν τρία API modules, `api_default`, `api_public` και `api_moderation`. Το `api_default` περιέχει μεθόδους που απαιτούν το κλειδί API σας, και το `api_public` περιλαμβάνει κλήσεις API που μπορούν να γίνουν απευθείας από έναν φυλλομετρητή/συσκευή κινητού κ.λπ. χωρίς πιστοποίηση. Το module `api_moderation` περιέχει μεθόδους για τον πίνακα ελέγχου του συντονιστή.

Το module `api_moderation` προσφέρει μια εκτεταμένη σειρά από ζωντανά και γρήγορα API συντονισμού. Κάθε μέθοδος `api_moderation` δέχεται μια παράμετρο `sso` και μπορεί να πιστοποιηθεί μέσω SSO ή ενός cookie συνεδρίας FastComments.com.