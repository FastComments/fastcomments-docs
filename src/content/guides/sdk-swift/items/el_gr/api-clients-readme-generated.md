The FastComments SDK provides three API clients:

### PublicAPI - Μέθοδοι Ασφαλείς για Πελάτες

The `PublicAPI` contains methods that are safe to call from client-side code (iOS/macOS apps). These methods:
- Do not require an API key → Δεν απαιτούν κλειδί API
- Can use SSO tokens for authentication → Μπορούν να χρησιμοποιήσουν διακριτικά SSO για έλεγχο ταυτότητας
- Are rate-limited per user/device → Έχουν περιορισμό ρυθμού ανά χρήστη/συσκευή
- Are suitable for end-user facing applications → Είναι κατάλληλες για εφαρμογές που προορίζονται για τελικούς χρήστες

**Παράδειγμα χρήσης**: Fetching and creating comments in your iOS app

### DefaultAPI - Μέθοδοι Πλευράς Διακομιστή

The `DefaultAPI` contains authenticated methods that require an API key. These methods:
- Require your FastComments API key → Απαιτούν το κλειδί API του FastComments
- Should ONLY be called from server-side code → Πρέπει να κλήνονται ΜΟΝΟ από κώδικα στην πλευρά του διακομιστή
- Provide full access to your FastComments data → Παρέχουν πλήρη πρόσβαση στα δεδομένα του FastComments
- Are rate-limited per tenant → Έχουν περιορισμό ρυθμού ανά ενοικιαστή

**Παράδειγμα χρήσης**: Administrative operations, bulk data export, user management

### ModerationAPI - Μέθοδοι Πίνακα Ελέγχου Συντονιστών

The `ModerationAPI` provides an extensive suite of live and fast moderation APIs. Every `ModerationAPI` method accepts an `sso` parameter and can authenticate via SSO or a FastComments.com session cookie.

**Παράδειγμα χρήσης**: Building a moderation experience for moderators of your community

**ΣΗΜΑΝΤΙΚΟ**: Never expose your API key in client-side code. API keys should only be used server-side.