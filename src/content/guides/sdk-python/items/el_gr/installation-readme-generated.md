---
### PyPI

```bash
pip install fastcomments
```

### Περιεχόμενα βιβλιοθήκης

Αυτή η βιβλιοθήκη περιέχει δύο modules: τον παραγόμενο API client και τη βασική βιβλιοθήκη Python που περιέχει χειρογραφικά βοηθητικά εργαλεία για να διευκολύνει την εργασία με το API, συμπεριλαμβανομένης της υποστήριξης SSO.

- [Τεκμηρίωση βιβλιοθήκης API Client](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Τεκμηρίωση βασικής βιβλιοθήκης, συμπεριλαμβανομένων παραδειγμάτων SSO](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Δημόσια vs Ασφαλή APIs

Για τον API client, υπάρχουν δύο κλάσεις, `DefaultApi` και `PublicApi`. Η `DefaultApi` περιέχει μεθόδους που απαιτούν το API key σας, και η `PublicApi` περιέχει κλήσεις API που μπορούν να γίνουν απευθείας από έναν browser/κινητή συσκευή/κ.λπ. χωρίς έλεγχο ταυτότητας.
---