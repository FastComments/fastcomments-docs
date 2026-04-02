### Yorumlar

```swift
let imageUrl = try await sdk.uploadImage(imageData: jpegData, filename: "photo.jpg")
```

Yüklenen resmin URL dizesini döndürür.

### Akış Gönderileri

```swift
let mediaItem = try await feedSDK.uploadImage(imageData: jpegData, filename: "photo.jpg")

// Paralel olarak birden çok resmi yükle
let mediaItems = try await feedSDK.uploadImages(images: [
    (jpegData1, "photo1.jpg"),
    (jpegData2, "photo2.jpg")
])
```

---
---