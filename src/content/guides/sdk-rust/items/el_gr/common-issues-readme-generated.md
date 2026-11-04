### Σφάλματα 401 Unauthorized

Εάν λαμβάνετε σφάλματα 401 κατά τη χρήση του πιστοποιημένου API:

1. **Ελέγξτε το API key σας**: Βεβαιωθείτε ότι χρησιμοποιείτε το σωστό API key από το dashboard του FastComments
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

1. **Χρησιμοποιήστε secure mode για παραγωγή**: Χρησιμοποιείτε πάντα το `FastCommentsSSO::new_secure()` με το API key σας για παραγωγή
2. **Μόνο server-side**: Δημιουργήστε SSO tokens στον server σας, μην εκθέτετε ποτέ το API key σας σε clients
3. **Ελέγξτε τα δεδομένα χρήστη**: Βεβαιωθείτε ότι παρέχονται όλα τα απαιτούμενα πεδία (id, email, username)

### Σφάλματα Async Runtime

Το SDK χρησιμοποιεί το tokio για ασύγχρονες λειτουργίες. Βεβαιωθείτε ότι:

1. Προσθέτετε το tokio στις εξαρτήσεις σας:
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

2. Χρησιμοποιείτε το tokio runtime:
```rust
#[tokio::main]
async fn main() {
    // Ο ασύγχρονος κώδικάς σας εδώ
}
```