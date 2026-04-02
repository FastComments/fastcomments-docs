### Коментари

```swift
let imageUrl = try await sdk.uploadImage(imageData: jpegData, filename: "photo.jpg")
```

Враћа URL као низ отпремљене слике.

### Објаве фида

```swift
let mediaItem = try await feedSDK.uploadImage(imageData: jpegData, filename: "photo.jpg")

// Отпремите више слика паралелно
let mediaItems = try await feedSDK.uploadImages(images: [
    (jpegData1, "photo1.jpg"),
    (jpegData2, "photo2.jpg")
])
```

---
---