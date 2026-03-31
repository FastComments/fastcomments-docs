### Σχόλια

```swift
let imageUrl = try await sdk.uploadImage(imageData: jpegData, filename: "photo.jpg")
```

Επιστρέφει τη συμβολοσειρά URL της ανεβασμένης εικόνας.

### Αναρτήσεις ροής

```swift
let mediaItem = try await feedSDK.uploadImage(imageData: jpegData, filename: "photo.jpg")

// Ανέβασμα πολλαπλών εικόνων παράλληλα
let mediaItems = try await feedSDK.uploadImages(images: [
    (jpegData1, "photo1.jpg"),
    (jpegData2, "photo2.jpg")
])
```

---
---