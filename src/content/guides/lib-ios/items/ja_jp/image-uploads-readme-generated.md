### コメント

```swift
let imageUrl = try await sdk.uploadImage(imageData: jpegData, filename: "photo.jpg")
```

アップロードされた画像のURL文字列を返します。

### フィード投稿

```swift
let mediaItem = try await feedSDK.uploadImage(imageData: jpegData, filename: "photo.jpg")

// 複数の画像を並列にアップロードする
let mediaItems = try await feedSDK.uploadImages(images: [
    (jpegData1, "photo1.jpg"),
    (jpegData2, "photo2.jpg")
])
```

---
---