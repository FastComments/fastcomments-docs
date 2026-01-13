The FastComments Rust SDK αποτελείται από αρκετές μονάδες:

- **Client Module** - Αυτοματοποιημένος πελάτης API για τα FastComments REST APIs
  - Πλήρεις ορισμοί τύπων για όλα τα μοντέλα API
  - Τόσο πιστοποιημένα (`DefaultApi`) όσο και δημόσια (`PublicApi`) endpoints
  - Πλήρης υποστήριξη async/await με tokio
  - Δείτε [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) για λεπτομερή τεκμηρίωση API

- **SSO Module** - Εργαλεία Single Sign-On στην πλευρά του διακομιστή
  - Ασφαλής δημιουργία token για πιστοποίηση χρήστη
  - Υποστήριξη τόσο για απλές όσο και για ασφαλείς λειτουργίες SSO
  - Υπογραφή token βασισμένη σε HMAC-SHA256

- **Core Types** - Κοινόχρηστοι ορισμοί τύπων και βοηθητικά εργαλεία
  - Μοντέλα σχολίων και δομές μεταδεδομένων
  - Διαμορφώσεις χρήστη και tenant
  - Βοηθητικές συναρτήσεις για κοινές λειτουργίες