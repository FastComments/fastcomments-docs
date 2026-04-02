### Коментари

```swift
let imageUrl = try await sdk.uploadImage(imageData: jpegData, filename: "photo.jpg")
```

Връща низ със URL адреса на каченото изображение.

### Публикации в емисията

```swift
let mediaItem = try await feedSDK.uploadImage(imageData: jpegData, filename: "photo.jpg")

// Качване на няколко изображения паралелно
let mediaItems = try await feedSDK.uploadImages(images: [
    (jpegData1, "photo1.jpg"),
    (jpegData2, "photo2.jpg")
])
```

---
---