### 401 Σφάλματα μη εξουσιοδότησης

Εάν λαμβάνετε σφάλματα 401 κατά τη χρήση του αυθεντικοποιημένου API:

1. **Ελέγξτε το API key σας**: Βεβαιωθείτε ότι χρησιμοποιείτε το σωστό API key από τον πίνακα ελέγχου FastComments  
2. **Επαληθεύστε το tenant ID**: Βεβαιωθείτε ότι το tenant ID ταιριάζει με τον λογαριασμό σας  
3. **Μορφή API key**: Το API key πρέπει να περαστεί στο Configuration:

```rust
let mut config = Configuration::new();
config.api_key = Some(ApiKey {
    prefix: None,
    key: "YOUR_API_KEY".to_string(),
});
```

### Προβλήματα SSO Token

Εάν τα SSO tokens δεν λειτουργούν:

1. **Use secure mode for production**: Always use `FastCommentsSSO::new_secure()` with your API key for production  
2. **Μόνο στην πλευρά του διακομιστή**: Δημιουργείτε τα SSO tokens στον διακομιστή σας, μην αποκαλύπτετε ποτέ το API key σας στους πελάτες  
3. **Ελέγξτε τα δεδομένα χρήστη**: Βεβαιωθείτε ότι παρέχονται όλα τα απαιτούμενα πεδία (id, email, username)  

### Σφάλματα χρόνου εκτέλεσης Async

Το SDK χρησιμοποιεί το tokio για ασύγχρονες λειτουργίες. Βεβαιωθείτε ότι:

1. Προσθέστε το tokio στις εξαρτήσεις σας:
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

2. Χρησιμοποιήστε το tokio runtime:
```rust
#[tokio::main]
async fn main() {
    // Εδώ βάλτε τον ασύγχρονο κώδικά σας
}
```