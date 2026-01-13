---
Το FastComments Rust SDK αποτελείται από αρκετές ενότητες:

- **Client Module** - Αυτόματος πελάτης API που δημιουργείται για τα FastComments REST APIs
  - Πλήρεις ορισμοί τύπων για όλα τα μοντέλα API
  - Υποστήριξη τόσο για πιστοποιημένα (`DefaultApi`) όσο και για δημόσια (`PublicApi`) endpoints
  - Πλήρης υποστήριξη async/await με tokio
  - Δείτε [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) για λεπτομερή τεκμηρίωση του API

- **SSO Module** - Εργαλεία Single Sign-On στην πλευρά του server
  - Ασφαλής δημιουργία token για έλεγχο ταυτότητας χρηστών
  - Υποστήριξη τόσο απλών όσο και ασφαλών λειτουργιών SSO
  - Υπογραφή token με βάση το HMAC-SHA256

- **Core Types** - Κοινόχρηστοι ορισμοί τύπων και βοηθητικά εργαλεία
  - Μοντέλα σχολίων και δομές μεταδεδομένων
  - Ρυθμίσεις χρηστών και tenants
  - Βοηθητικές συναρτήσεις για κοινές λειτουργίες
---