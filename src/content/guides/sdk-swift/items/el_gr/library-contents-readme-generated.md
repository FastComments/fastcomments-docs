Το FastComments Swift SDK αποτελείται από αρκετές μονάδες:

- **Client Module** - Πελάτης API για τα FastComments REST APIs
  - Πλήρεις ορισμοί τύπων για όλα τα μοντέλα API
  - Αυθεντικοποιημένες (`DefaultAPI`), δημόσιες (`PublicAPI`) και μεθόδους συντονισμού (`ModerationAPI`)
  - Πλήρης υποστήριξη async/await
  - Δείτε το [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) για λεπτομερή τεκμηρίωση API

- **SSO Module** - Εργαλεία Single Sign-On από την πλευρά του διακομιστή
  - Ασφαλής δημιουργία token για την αυθεντικοποίηση χρηστών
  - Υποστήριξη τόσο των απλών όσο και των ασφαλών λειτουργιών SSO
  - Υπογραφή token βάσει HMAC-SHA256 χρησιμοποιώντας CryptoKit