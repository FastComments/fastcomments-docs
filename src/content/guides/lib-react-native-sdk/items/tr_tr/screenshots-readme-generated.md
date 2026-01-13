#### Tema: Erebus
![Tema: Erebus](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-erebus.PNG)
#### Tema: Default
![Tema: Default](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-default.PNG)
#### Resim Desteği ile Yerel WYSIWYG Düzenleyici!
![Resim Desteği ile Yerel WYSIWYG Düzenleyici](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/native-wysiwyg.PNG)

### Zengin Metin Düzenleyici

Bu kütüphane zengin metin düzenleme işlevselliği için 10tap editörünü kullanır; bu, güçlü bir WYSIWYG düzenleme deneyimi sağlar.

### Yapılandırma Seçenekleri

Bu kütüphane, web uygulamasıyla aynı şekilde [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts) içinde tanımlanan tüm yapılandırma seçeneklerini desteklemeyi amaçlar.

### FastComments Kavramları

Başlamak için bilmeniz gereken temel kavramlar `tenantId` ve `urlId`'dir. `tenantId`, FastComments.com hesap tanımlayıcınızdır. `urlId`, yorum dizilerinin bağlanacağı yerdir. Bu bir sayfa URL'si, bir ürün id'si, bir makale id'si vb. olabilir.

### Kullanıcı Bildirimleri

FastComments [birçok senaryo](https://docs.fastcomments.com/guide-notifications.html) için bildirimleri destekler. Bildirimler yapılandırılabilir, küresel olarak veya bir bildirim/yorum düzeyinde tercih dışı bırakılabilir ve kullanıcıların belirli bir sayfanın veya makalenin dizilerine abone olabilmesi için sayfa düzeyinde abonelikleri destekler.

Örneğin, kullanıcıyı kimlik doğrulamak için Secure SSO kullanmak ve ardından okunmamış bildirimleri periyodik olarak sorgulayıp kullanıcıya iletmek mümkündür.

Okunmamış kullanıcı bildirimlerini alıp çevirmek için örneğe bakın: [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx).

### Gif Tarayıcı

Varsayılan olarak, hiçbir resim veya gif seçimi etkin değildir. Resim ve gif yüklemelerini nasıl destekleyeceğinizi görmek için bkz. [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx). Bu kütüphanede aramaları ve sağlanan görüntüleri anonimleştiren bir Gif Tarayıcı vardır; onu kullanmanız yeterlidir.

### Performans

Herhangi bir performans sorunu tespit ederseniz, kullanılan cihaz dahil tekrar üretmek için bir örnek içeren bir ticket açın lütfen. Performans, tüm FastComments kütüphanelerinin birinci sınıf önceliğidir.