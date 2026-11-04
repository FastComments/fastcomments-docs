### Σφάλματα 401 Unauthorized

Εάν λαμβάνετε σφάλματα 401 κατά τη χρήση του πιστοποιημένου API:

1. **Ελέγξτε το API key σας**: Βεβαιωθείτε ότι χρησιμοποιείτε το σωστό API key από το dashboard του FastComments
2. **Επαληθεύστε το tenant ID**: Βεβαιωθείτε ότι το tenant ID ταιριάζει με τον λογαριασμό σας
3. **Μορφή API key**: Το API key πρέπει να οριστεί ως κεφαλίδα `x-api-key` στην κοινή διαμόρφωση:

```swift
FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "YOUR_API_KEY"
```

4. **Χρήση λάθος API**: Βεβαιωθείτε ότι χρησιμοποιείτε το `DefaultAPI` (όχι το `PublicAPI`) για πιστοποιημένες κλήσεις

### Προβλήματα SSO Token

Εάν τα SSO tokens δεν λειτουργούν:

1. **Χρησιμοποιήστε secure mode για παραγωγή**: Χρησιμοποιείτε πάντα το `FastCommentsSSO.createSecure()` με το API key σας για παραγωγή
2. **Μόνο από τον server**: Δημιουργήστε secure SSO tokens στον server σας, μην εκθέτετε ποτέ το API key σας σε clients
3. **Ελέγξτε τα δεδομένα χρήστη**: Βεβαιωθείτε ότι παρέχονται όλα τα απαιτούμενα πεδία (id, email, username)
4. **Λήξη token**: Τα secure SSO tokens περιλαμβάνουν χρονική σήμανση και ενδέχεται να λήξουν. Δημιουργήστε νέα tokens όπως χρειάζεται.

### Σφάλματα SSL/TLS

Εάν αντιμετωπίζετε σφάλματα SSL/TLS:

1. Βεβαιωθείτε ότι το Info.plist της εφαρμογής σας επιτρέπει συνδέσεις HTTPS προς το fastcomments.com
2. Ελέγξτε ότι δεν χρησιμοποιείτε εξαιρέσεις App Transport Security που ενδέχεται να μπλοκάρουν τη σύνδεση