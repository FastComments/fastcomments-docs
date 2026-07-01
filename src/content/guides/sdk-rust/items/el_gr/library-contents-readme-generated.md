---
Το FastComments Rust SDK αποτελείται από αρκετές μονάδες:

- **Μονάδα Πελάτη** - API client για τα FastComments REST APIs
  - Πλήρεις ορισμοί τύπων για όλα τα μοντέλα API
  - Τρεις API clients που καλύπτουν όλες τις μεθόδους FastComments:
    - `default_api` (**DefaultApi**) - μέθοδοι με αυθεντικοποίηση μέσω API-key για χρήση στο server-side
    - `public_api` (**PublicApi**) - δημόσιες, χωρίς API-key μέθοδοι που είναι ασφαλείς για κλήση από browsers και mobile apps
    - `moderation_api` (**ModerationApi**) - εκτενής σύνολο ζωντανών και γρήγορων APIs συντονισμού. Κάθε μέθοδος Moderation δέχεται μια παράμετρο `sso` και μπορεί να αυθεντικοποιηθεί μέσω SSO ή ενός cookie συνεδρίας FastComments.com.
  - Πλήρης υποστήριξη async/await με tokio
  - Δείτε το [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) για λεπτομερή τεκμηρίωση API

- **Μονάδα SSO** - Βοηθητικά εργαλεία Server-side Single Sign-On
  - Ασφαλής δημιουργία token για αυθεντικοποίηση χρήστη
  - Υποστήριξη τόσο απλών όσο και ασφαλών λειτουργιών SSO
  - Υπογραφή token με βάση HMAC‑SHA256

- **Κύριοι Τύποι** - Κοινές ορισμοί τύπων και βοηθητικά εργαλεία
  - Μοντέλα σχολίων και δομές μεταδεδομένων
  - Ρυθμίσεις χρήστη και μισθωτή
  - Βοηθητικές λειτουργίες για κοινές εργασίες
---