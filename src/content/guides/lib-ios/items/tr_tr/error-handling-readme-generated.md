---
SDK yöntemleri `FastCommentsError` fırlatır; bu, `LocalizedError` ile uyumludur:

```swift
do {
    try await sdk.load()
} catch let error as FastCommentsError {
    print(error.translatedError ?? error.reason ?? "Unknown error")
} catch {
    print(error.localizedDescription)
}
```

`FastCommentsError` özellikleri:

- `code` -- API'den gelen hata kodu
- `reason` -- İngilizce hata açıklaması
- `translatedError` -- sunucu tarafından sağlanan yerelleştirilmiş hata mesajı

Engelleme hataları ayrıca `sdk.blockingErrorMessage` aracılığıyla otomatik olarak ortaya çıkarılır; yerleşik görünümler bunları kullanıcıya gösterir.

---
---