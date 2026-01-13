### Yayın Kimlikleri

Bazı API çağrılarında `broadcastId` geçirmeniz gerektiğini göreceksiniz. Olayları aldığınızda bu ID size geri gönderilir, böylece istemcide değişiklikleri optimistçe uygulamayı planlıyorsanız olayı görmezden gelmeniz gerektiğini bilirsiniz (bunu muhtemelen yapmak isteyeceksiniz çünkü en iyi kullanıcı deneyimini sağlar). Buraya bir UUID gönderin. Bu ID, bir oturum içinde iki kez tekrar etmeyecek kadar benzersiz olmalıdır.

```swift
let broadcastId = UUID().uuidString
```