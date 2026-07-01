### 401 Unauthorized Errors

If you're getting 401 errors when using the authenticated API:

1. **Check your API key**: Ensure you're using the correct API key from your FastComments dashboard  
   → **Ελέγξτε το κλειδί API σας**: Βεβαιωθείτε ότι χρησιμοποιείτε το σωστό κλειδί API από τον πίνακα ελέγχου FastComments σας
2. **Verify the tenant ID**: Make sure the tenant ID matches your account  
   → **Επιβεβαιώστε το tenant ID**: Βεβαιωθείτε ότι το tenant ID ταιριάζει με τον λογαριασμό σας
3. **API key format**: The API key should be set as the `x-api-key` header on the shared configuration:  
   → **Μορφή κλειδιού API**: Το κλειδί API πρέπει να οριστεί ως η κεφαλίδα `x-api-key` στη κοινή διαμόρφωση:

```swift
FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "YOUR_API_KEY"
```

4. **Using the wrong API**: Make sure you're using `DefaultAPI` (not `PublicAPI`) for authenticated calls  
   → **Χρήση λανθασμένου API**: Βεβαιωθείτε ότι χρησιμοποιείτε το `DefaultAPI` (όχι το `PublicAPI`) για αυθεντοποιημένες κλήσεις

### SSO Token Issues

If SSO tokens aren't working:

1. **Use secure mode for production**: Always use `FastCommentsSSO.createSecure()` with your API key for production  
   → **Χρησιμοποιήστε ασφαλή λειτουργία για παραγωγή**: Πάντα χρησιμοποιείτε το `FastCommentsSSO.createSecure()` με το κλειδί API σας για παραγωγή
2. **Server-side only**: Generate secure SSO tokens on your server, never expose your API key to clients  
   → **Μόνο στον διακομιστή**: Δημιουργήστε ασφαλή tokens SSO στον διακομιστή σας, μην εκθέτετε ποτέ το κλειδί API στους πελάτες
3. **Check user data**: Ensure all required fields (id, email, username) are provided  
   → **Ελέγξτε τα δεδομένα χρήστη**: Βεβαιωθείτε ότι όλα τα απαιτούμενα πεδία (id, email, username) παρέχονται
4. **Token expiration**: Secure SSO tokens include a timestamp and may expire. Generate fresh tokens as needed.  
   → **Λήξη token**: Τα ασφαλή tokens SSO περιλαμβάνουν χρονική σήμανση και μπορεί να λήξουν. Δημιουργήστε νέες εκδόσεις token όταν χρειάζεται.

### SSL/TLS Errors

If you encounter SSL/TLS errors:

1. Ensure your app's Info.plist allows HTTPS connections to fastcomments.com  
   → **Βεβαιωθείτε ότι το Info.plist της εφαρμογής σας επιτρέπει συνδέσεις HTTPS προς το fastcomments.com**
2. Check that you're not using App Transport Security exceptions that might block the connection  
   → **Ελέγξτε ότι δεν χρησιμοποιείτε εξαιρέσεις App Transport Security που ίσως να μπλοκάρουν τη σύνδεση**