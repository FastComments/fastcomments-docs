#### Tema: Erebus
![Tema: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### Tema: Default
![Tema: Default](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### Yerel WYSIWYG Düzenleyici — Görsel Desteği Var!
![Yerel WYSIWYG Düzenleyici — Görsel Desteği Var](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### Zengin Metin Düzenleyici

Bu kütüphane, zengin metin düzenleme işlevselliği için 10tap düzenleyicisini kullanır; bu düzenleyici güçlü bir WYSIWYG düzenleme deneyimi sunar.

### Yapılandırma Seçenekleri

Bu kütüphane, web uygulaması ile aynı şekilde [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts) içinde tanımlanan tüm yapılandırma seçeneklerini desteklemeyi amaçlar.

### FastComments Kavramları

Başlamak için bilmeniz gereken ana kavramlar `tenantId` ve `urlId`'dir. `tenantId`, FastComments.com hesap tanımlayıcınızdır. `urlId` ise yorum dizilerinin bağlanacağı yerdir. Bu bir sayfa URL'si, bir ürün kimliği, bir makale kimliği vb. olabilir.

### Kullanıcı Bildirimleri

FastComments [birçok senaryo için](https://docs.fastcomments.com/guide-notifications.html) bildirimleri destekler. Bildirimler yapılandırılabilir, genel veya bildirim/yorum düzeyinde tercih dışı bırakılabilir ve kullanıcıların belirli bir sayfa veya makalenin dizilerine abone olabilmesini sağlayan sayfa düzeyinde abonelikleri destekler.

Örneğin, kullanıcıyı kimlik doğrulamak için Secure SSO kullanmak ve ardından okunmamış bildirimleri periyodik olarak sorgulayıp bunları kullanıcıya itmek mümkündür.

Okunmamış kullanıcı bildirimlerini nasıl alıp çevireceğinizi görmek için [örnek AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) dosyasına bakın.

### GIF Tarayıcısı

Varsayılan olarak, herhangi bir görsel veya GIF seçimi etkin değildir. Görsel ve GIF yüklemelerini nasıl destekleyeceğinizi görmek için [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) dosyasına bakın. Bu kütüphanede aramaları ve sağlanan görselleri anonimleştiren bir GIF Tarayıcısı bulunuyor; tek yapmanız gereken onu kullanmaktır.

### Performans

Herhangi bir performans sorunu tespit ederseniz, kullandığınız cihazı da içeren yeniden üretilebilir bir örnekle birlikte bir destek bileti açın lütfen. Performans, tüm FastComments kütüphanelerinin birinci sınıf önceliğidir.