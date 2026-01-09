### Using Nimble

```bash
nimble install fastcomments
```

### Building from Source

```bash
nimble build
```

### Library Contents

Αυτή η βιβλιοθήκη περιλαμβάνει τον δημιουργημένο πελάτη API και τα βοηθητικά εργαλεία SSO για να διευκολύνουν τη χρήση του API.

- [Τεκμηρίωση βιβλιοθήκης πελάτη API](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Public vs Secured APIs

Για τον πελάτη API, υπάρχουν δύο modules API, `api_default` και `api_public`. Το `api_default` περιέχει μεθόδους που απαιτούν το κλειδί API σας, και το `api_public` περιέχει κλήσεις API που μπορούν να γίνουν απευθείας από ένα πρόγραμμα περιήγησης/κινητή συσκευή/κ.λπ. χωρίς πιστοποίηση.