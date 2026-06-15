---
Avatarlar, iç içe yanıtlar, oylar ve yerleşik zengin metin düzenleyicisi ile canlı, dallanmış yorumlar; ayrıca koyu tema ve canlı sohbet ön ayarı (burada `react-native-web` ile render edilmiş olarak gösterilmiştir):

<table>
  <tr>
    <td align="center"><b>Canlı Yorumlama</b><br/><img src="./demo-screenshots/light.png" width="260" alt="Canlı yorumlama, açık tema"/></td>
    <td align="center"><b>Koyu Tema</b><br/><img src="./demo-screenshots/dark.png" width="260" alt="Canlı yorumlama, koyu tema"/></td>
    <td align="center"><b>Canlı Sohbet</b><br/><img src="./demo-screenshots/chat.png" width="260" alt="Canlı sohbet ön ayarı"/></td>
  </tr>
</table>

### Zengin Metin Düzenleyicisi

Bu kütüphane zengin metin düzenleme için [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) kullanır; bu, güçlü bir WYSIWYG düzenleme deneyimi sağlar. Aynı düzenleyici iOS, Android ve web (via `react-native-web`) için güç sağlar, bu yüzden düzenleyici tek bir uygulama ile her platformda tutarlı davranır.

`react-native-enriched`, native tarafında React Native Yeni Mimarisi (Fabric) gerektirir (RN 0.76'dan beri varsayılan, RN 0.72-0.75'te isteğe bağlı) ve paket `exports` koşullarını çözen bir bundler gerektirir. Bu SDK RN 0.81 / React 19 üzerinde geliştirilmiş ve test edilmiştir. Aynı düzenleyici web üzerinde `react-native-web` aracılığıyla da çalışır; enriched düzenleyicinin web derlemesi upstream'de hâlâ deneysel olarak işaretlenmiştir.

### Widget'lar

SDK, FastComments Android SDK'sını yansıtan üç widget ile gelir:

- `FastCommentsLiveCommenting` - oylar, yanıtlar, sayfalama, bahsetmeler, bildirimler ve canlı güncellemeler içeren dallanmış yorumlama.
- `FastCommentsLiveChat` - aynı motor üzerinde bir sohbet ön ayarı: yeni mesajların aşağıda göründüğü kronolojik mesajlar, listenin altında düzenleyici, canlı başlık şeridi (bağlantı noktası + kullanıcı sayısı), yukarı kaydırarak yüklenen sonsuz geçmiş, yeni mesajlara otomatik kaydırma, oy veya yanıt dallanması yok. Her ön ayar `config` aracılığıyla ezilebilir.
- `FastCommentsFeed` - gönderi düzenleyicisi, medya, reaksiyonlar, takipler ve canlı yeni gönderi pankartları içeren sosyal bir akış.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Tema

Varsayılan görünüm, bir dizi anlamsal tasarım token'ından (`FastCommentsTheme`) üretilir: renkler, boşluklar, radius, yazı tipi boyutları, yazı ağırlıkları ve avatar boyutları. Kısmi token geçersiz kılmaları (türlenmiş `FastCommentsThemeOverrides`) herhangi bir widget'taki `theme` prop'u üzerinden iletin ve tüm stil ağacı tutarlı şekilde yeniden stilize edilir:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

Koyu mod, bir token seti uzağınızdadır:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

`styles` prop'u hâlâ cerrahi kontrol için ham bir `IFastCommentsStyles` ağacı kabul eder. `theme` ve `styles` birlikte sağlandığında, açıkça belirtilen stiller temalı ağacın üzerinde üstünlük sağlar; yalnızca `styles` sağlandığında ise varsayılanları tamamen değiştirir (orijinal davranış, böylece mevcut entegrasyonlar ve temalar etkilenmez). `setupDarkModeSkin`, `theme` prop'una tercih edilmesi lehine kullanımdan kaldırılmıştır.

### Yapılandırma Seçenekleri

Bu kütüphane, web uygulaması gibi [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts) içinde tanımlanan tüm yapılandırma seçeneklerini desteklemeyi amaçlar.

Bunların ötesinde, React Native `FastCommentsRNConfig` aracılığıyla birkaç SDK-özgü seçenek ekler:

- `hideTopBar` - yazma alanının üzerinde gösterilen oturum açmış kullanıcı / bildirim zili şeridini gizle.
- `usePressToEdit` - bir yorumun menüsünü açmak için yoruma basılı tutma eylemini kullan.
- `disableDownVoting` - aşağı oy butonlarını gizle.
- `renderCommentInline` - yorumcu bilgilerini yorum içeriğiyle aynı HTML bloğu içinde göster.
- `renderLikesToRight` - oy/beğeni alanını yorumun altına yerine sağ tarafına taşı.
- `renderDateBelowComment` - tarihi yorumun altında göster.
- `showLiveStatus` - yorumların üzerinde sohbet tarzı "Live" + kullanıcı sayısı başlık şeridini göster.
- `useInlineSubmitButton` - gönder düğmesini yazma alanı içinde bir simge olarak göster.
- `countAboveToggle` - `useShowCommentsToggle` ile birlikte, "Yorumları Göster" geçişinin üzerinde kaç yorumun render edileceği.
- `preserveFeedScrollPosition` - `FastCommentsFeed`, unmount/remount arasında kaydırma konumunu hatırlar (varsayılan true).

### FastComments Kavramları

Başlamak için bilmeniz gereken ana kavramlar `tenantId` ve `urlId`'dir. `tenantId`, FastComments.com hesap tanımlayıcınızdır. `urlId`, yorum dizilerinin bağlanacağı yerdir. Bu bir sayfa URL'si, bir ürün id'si, bir makale id'si vb. olabilir.

### Kullanıcı Bildirimleri

FastComments, [birçok senaryo için](https://docs.fastcomments.com/guide-notifications.html) bildirimleri destekler. Bildirimler yapılandırılabilir, küresel veya bildirim/yorum düzeyinde vazgeçilebilir ve kullanıcıların belirli bir sayfa veya makale dizilerine abone olabilmesi için sayfa düzeyinde abonelikleri destekler.

Örneğin, Secure SSO kullanarak kullanıcıyı kimlik doğrulamak ve ardından okunmamış bildirimleri periyodik olarak çekip kullanıcıya iletmek mümkündür.

Okunmamış kullanıcı bildirimlerini nasıl alıp çevireceğinizi görmek için [örnek AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) dosyasına bakın.

### Gif Tarayıcı

Varsayılan olarak, hiçbir resim veya gif seçimi etkin değildir. Resim ve gif yüklemelerini nasıl destekleyeceğinizi görmek için [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) dosyasına bakın. Bu kütüphanede aramaları ve sağlanan görüntüleri anonimleştiren bir Gif Tarayıcı vardır; kullanmanız yeterlidir.

### Performans

Herhangi bir performans sorunu tespit ederseniz, kullanılan cihaz da dahil olmak üzere yeniden üretmek için bir örnek içeren bir ticket açın. Performans, tüm FastComments kütüphanelerinin birinci sınıf önceliğidir.
---