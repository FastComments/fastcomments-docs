---
Το FastComments Swift SDK αποτελείται από αρκετές μονάδες:

- **Client Module** - Αυτο-γεννημένος πελάτης API για τα FastComments REST APIs
  - Πλήρεις ορισμοί τύπων για όλα τα μοντέλα API
  - Και τα αυθεντικοποιημένα (`DefaultAPI`) και τα δημόσια (`PublicAPI`) endpoints
  - Πλήρης υποστήριξη για async/await
  - Δείτε [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) για λεπτομερή τεκμηρίωση API

- **SSO Module** - Βοηθητικά εργαλεία Single Sign-On στην πλευρά του διακομιστή
  - Ασφαλής δημιουργία token για αυθεντικοποίηση χρηστών
  - Υποστήριξη τόσο απλών όσο και ασφαλών τρόπων λειτουργίας SSO
  - Υπογραφή token βασισμένη σε HMAC-SHA256 χρησιμοποιώντας CryptoKit
---