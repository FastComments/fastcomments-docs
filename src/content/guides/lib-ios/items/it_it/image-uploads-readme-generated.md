### Commenti

```swift
let imageUrl = try await sdk.uploadImage(imageData: jpegData, filename: "photo.jpg")
```

Restituisce la stringa URL dell'immagine caricata.

### Post del Feed

```swift
let mediaItem = try await feedSDK.uploadImage(imageData: jpegData, filename: "photo.jpg")

// Carica più immagini in parallelo
let mediaItems = try await feedSDK.uploadImages(images: [
    (jpegData1, "photo1.jpg"),
    (jpegData2, "photo2.jpg")
])
```