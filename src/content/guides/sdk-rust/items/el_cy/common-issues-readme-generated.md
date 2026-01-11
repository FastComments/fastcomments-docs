### Σφάλματα 401 — Μη εξουσιοδοτημένο

Αν λαμβάνετε σφάλματα 401 όταν χρησιμοποιείτε το authenticated API:

1. **Check your API key**: Βεβαιωθείτε ότι χρησιμοποιείτε το σωστό API key από τον πίνακα ελέγχου του FastComments
2. **Verify the tenant ID**: Βεβαιωθείτε ότι το tenant ID αντιστοιχεί στον λογαριασμό σας
3. **API key format**: Το API key πρέπει να περαστεί στο Configuration:

```rust
let mut config = Configuration::new();
config.api_key = Some(ApiKey {
    prefix: None,
    key: "YOUR_API_KEY".to_string(),
});
```

### Προβλήματα με τα SSO tokens

Αν τα SSO tokens δεν λειτουργούν:

1. **Use secure mode for production**: Χρησιμοποιείτε πάντα `FastCommentsSSO::new_secure()` με το API key σας για περιβάλλον παραγωγής
2. **Server-side only**: Δημιουργείτε τα SSO tokens στον διακομιστή σας, μην αποκαλύπτετε ποτέ το API key σας στους πελάτες
3. **Check user data**: Βεβαιωθείτε ότι όλα τα απαιτούμενα πεδία (id, email, username) έχουν παρασχεθεί

### Σφάλματα χρόνου εκτέλεσης (Async)

Το SDK χρησιμοποιεί το tokio για ασύγχρονες λειτουργίες. Βεβαιωθείτε ότι:

1. Προσθέστε το tokio στις εξαρτήσεις σας:
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

2. Χρησιμοποιήστε το runtime του tokio:
```rust
#[tokio::main]
async fn main() {
    // Βάλτε εδώ τον ασύγχρονο κώδικά σας
}
```