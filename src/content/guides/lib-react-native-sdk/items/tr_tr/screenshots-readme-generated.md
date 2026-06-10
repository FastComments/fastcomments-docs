#### Tema: Erebus
![Tema: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### Tema: Varsayılan
![Tema: Varsayılan](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### Yerel WYSIWYG Düzenleyici — Resim Desteğiyle!
![Yerel WYSIWYG Düzenleyici — Resim Desteğiyle](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### Zengin Metin Düzenleyicisi

Bu kütüphane zengin metin düzenleme için [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) kullanır; bu, güçlü bir WYSIWYG düzenleme deneyimi sağlar. Aynı düzenleyici iOS, Android ve web (via `react-native-web`) için kullanıldığından, içerik oluşturucu tek bir uygulama ile tüm platformlarda tutarlı davranır.

`react-native-enriched` yerelde React Native Yeni Mimarisi (Fabric) gerektirir ve paket `exports` koşullarını çözen bir bağlayıcıya ihtiyaç duyar (paket ihracatları ile Metro / RN 0.72+). Web desteği şu anda deneysel durumdadır.

### Yapılandırma Seçenekleri

Bu kütüphane, web uygulamasıyla aynı şekilde [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts) içinde tanımlanan tüm yapılandırma seçeneklerini desteklemeyi hedefler.

### FastComments Kavramları

Başlamak için dikkat edilmesi gereken ana kavramlar `tenantId` ve `urlId`'dir. `tenantId`, FastComments.com hesap tanımlayıcınızdır. `urlId`, yorum dizilerinin bağlanacağı yerdir. Bu bir sayfa URL'si, bir ürün kimliği, bir makale kimliği vb. olabilir.

### Kullanıcı Bildirimleri

FastComments, [birçok senaryo](https://docs.fastcomments.com/guide-notifications.html) için bildirimleri destekler. Bildirimler yapılandırılabilir, küresel olarak veya bildirim/yorum seviyesinde tercih dışı bırakılabilir ve kullanıcıların belirli bir sayfa veya makalenin dizilerine abone olabilmesi için sayfa düzeyinde abonelikleri destekler.

Örneğin, kullanıcıyı kimlik doğrulamak için Secure SSO kullanmak ve ardından okunmamış bildirimler için periyodik olarak sorgulama yapıp bunları kullanıcıya iletmek mümkündür.

Okunmamış kullanıcı bildirimlerini alıp çevirmekle ilgili nasıl yapılacağını görmek için [örnek AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) dosyasına bakın.

### GIF Tarayıcısı

Varsayılan olarak, resim veya gif seçimi etkin değildir. Resim ve gif yüklemelerini nasıl destekleyeceğinizi görmek için [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) dosyasına bakın. Bu kütüphanede aramaları ve sağlanan görselleri anonimleştiren bir GIF Tarayıcısı bulunmaktadır; onu kullanmanız yeterlidir.

### Performans

Herhangi bir performans sorunu tespit ederseniz, kullanılan cihaz da dahil olmak üzere tekrar üretilebilecek bir örnek ile bir bildirim açın lütfen. Performans, tüm FastComments kütüphanelerinde birinci sınıf bir önceliktir.