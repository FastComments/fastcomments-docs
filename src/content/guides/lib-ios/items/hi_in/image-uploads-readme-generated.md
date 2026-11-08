### कमेंट्स

```swift
let imageUrl = try await sdk.uploadImage(imageData: jpegData, filename: "photo.jpg")
```

अपलोड की गई इमेज का URL स्ट्रिंग रिटर्न करता है।

### फ़ीड पोस्ट्स

```swift
let mediaItem = try await feedSDK.uploadImage(imageData: jpegData, filename: "photo.jpg")

// एक साथ कई इमेज पैरेलल में अपलोड करें
let mediaItems = try await feedSDK.uploadImages(images: [
    (jpegData1, "photo1.jpg"),
    (jpegData2, "photo2.jpg")
])
```

---
---