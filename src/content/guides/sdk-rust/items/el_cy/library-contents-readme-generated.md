The FastComments Rust SDK αποτελείται από αρκετές ενότητες:

- **Client Module** - Πελάτης API για τα FastComments REST APIs
  - Πλήρεις ορισμοί τύπων για όλα τα μοντέλα API
  - Τρεις πελάτες API που καλύπτουν όλες τις μεθόδους του FastComments:
    - `default_api` (**DefaultApi**) - Μέθοδοι που ελέγχονται με API-key για χρήση στην πλευρά του διακομιστή
    - `public_api` (**PublicApi**) - δημόσιες, χωρίς API-key μέθοδοι που είναι ασφαλείς για κλήση από προγράμματα περιήγησης και κινητές εφαρμογές
    - `moderation_api` (**ModerationApi**) - μια εκτενής σειρά από ζωντανά και γρήγορα APIs συντονισμού. Κάθε μέθοδος Moderation δέχεται μια παράμετρο `sso` και μπορεί να πιστοποιηθεί μέσω SSO ή ενός cookie συνεδρίας FastComments.com.
  - Πλήρης υποστήριξη async/await με tokio
  - Δείτε το [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) για λεπτομερή τεκμηρίωση API

- **SSO Module** - Εργαλεία Single Sign-On στην πλευρά του διακομιστή
  - Ασφαλής δημιουργία token για πιστοποίηση χρήστη
  - Υποστήριξη για απλούς και ασφαλείς τρόπους SSO
  - Υπογραφή token με βάση HMAC-SHA256

- **Core Types** - Κοινόχρηστοι ορισμοί τύπων και βοηθητικές λειτουργίες
  - Μοντέλα σχολίων και δομές μεταδεδομένων
  - Ρυθμίσεις χρήστη και ενοικιαστή
  - Βοηθητικές συναρτήσεις για κοινές λειτουργίες