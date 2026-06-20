Το FastComments Swift SDK αποτελείται από αρκετές μονάδες:

- **Μονάδα Πελάτη** - Πελάτης API για τα REST APIs του FastComments
  - Πλήρεις ορισμοί τύπων για όλα τα μοντέλα API
  - Μέθοδοι για αυθεντικοποίηση (`DefaultAPI`), δημόσιες (`PublicAPI`) και για moderation (`ModerationAPI`)
  - Πλήρης υποστήριξη async/await
  - Δείτε [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) για λεπτομερή τεκμηρίωση του API

- **Μονάδα SSO** - Εργαλεία Single Sign-On από την πλευρά του διακομιστή
  - Ασφαλής δημιουργία token για την πιστοποίηση χρήστη
  - Υποστήριξη τόσο απλού όσο και ασφαλούς τρόπου λειτουργίας SSO
  - Υπογραφή token βάσει HMAC-SHA256 χρησιμοποιώντας CryptoKit