Bir SDK örneği ile işiniz bittiğinde (örn. görünüm kapatıldığında), WebSocket bağlantısını kapatmak ve arka plan görevlerini iptal etmek için `cleanup()` çağırın:

```swift
sdk.cleanup()
```

SwiftUI'nin `@StateObject` ile yönetilen görünümler için, bu genellikle `.onDisappear` içinde veya görünüm bellekte serbest bırakıldığında çağrılır.