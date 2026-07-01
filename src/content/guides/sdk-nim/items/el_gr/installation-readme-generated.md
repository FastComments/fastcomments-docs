### Χρήση Nimble

```bash
nimble install fastcomments
```

### Δόμηση από Πηγή

```bash
nimble build
```

### Περιεχόμενα Βιβλιοθήκης

Αυτή η βιβλιοθήκη περιέχει τον δημιουργημένο πελάτη API και τα εργαλεία SSO για να διευκολύνουν την εργασία με το API.

- [Τεκμηρίωση Βιβλιοθήκης Πελάτη API](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Δημόσια vs Προστατευμένα APIs

Για τον πελάτη API, υπάρχουν τρία API modules, `api_default`, `api_public` και `api_moderation`. Το `api_default` περιέχει μεθόδους που απαιτούν το κλειδί API σας, και το `api_public` περιέχει κλήσεις API που μπορούν να γίνουν απευθείας από έναν φυλλομετρητή/συσκευή κινητού κ.λπ. χωρίς πιστοποίηση. Το module `api_moderation` περιέχει μεθόδους για τον πίνακα ελέγχου του συντονιστή.

Το `api_moderation` module παρέχει μια εκτενή σειρά ζωντανών και γρήγορων APIs συντονισμού. Κάθε μέθοδος `api_moderation` δέχεται μια παράμετρο `sso` και μπορεί να πιστοποιηθεί μέσω SSO ή ενός cookie συνεδρίας FastComments.com.