The FastComments Swift SDK αποτελείται από αρκετές μονάδες:

- **Client Module** - Αυτόματος πελάτης API που δημιουργείται για τα FastComments REST APIs
  - Πλήρεις ορισμοί τύπων για όλα τα μοντέλα API
  - Και τα εξουσιοδοτημένα (`DefaultAPI`) και τα δημόσια (`PublicAPI`) endpoints
  - Πλήρης υποστήριξη για async/await
  - Δείτε [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) για λεπτομερή τεκμηρίωση API

- **SSO Module** - Εργαλεία Single Sign-On από την πλευρά του διακομιστή
  - Ασφαλής δημιουργία token για επαλήθευση ταυτότητας χρηστών
  - Υποστήριξη τόσο του απλού όσο και του ασφαλούς τρόπου SSO
  - Υπογραφή token βασισμένη σε HMAC-SHA256 χρησιμοποιώντας CryptoKit