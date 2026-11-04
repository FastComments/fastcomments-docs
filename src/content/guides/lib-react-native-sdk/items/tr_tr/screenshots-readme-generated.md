Avatarlar, iç içe yanıtlar, oylar ve yerleşik zengin metin editörü ile canlı, dallanmış yorum özelliği; ayrıca koyu tema ve canlı sohbet ön ayarı (burada `react-native-web` aracılığıyla render edilmektedir):

<table>
  <tr>
    <td align="center"><b>Canlı Yorumlama</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-light.png" width="260" alt="Canlı yorumlama, açık tema"/></td>
    <td align="center"><b>Koyu Tema</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-dark.png" width="260" alt="Canlı yorumlama, koyu tema"/></td>
    <td align="center"><b>Canlı Sohbet</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-chat.png" width="260" alt="Canlı sohbet ön ayarı"/></td>
  </tr>
</table>

### Zengin Metin Editörü

Bu kütüphane, güçlü bir WYSIWYG düzenleme deneyimi sunan zengin metin düzenleme için [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched-html) kullanır. Aynı editör iOS, Android ve web'i (`react-native-web` aracılığıyla) destekler, böylece tek bir uygulamayla besteci her platformda tutarlı şekilde davranır.

`react-native-enriched`, native tarafta React Native New Architecture (Fabric) gerektirir (RN 0.76'dan beri varsayılan, RN 0.72-0.75'te isteğe bağlı) ve paket `exports` koşullarını çözümleyen bir bundler gerektirir. Bu SDK, RN 0.81 / React 19'a karşı geliştirilmiş ve test edilmiştir. Aynı editör `react-native-web` üzerinden web'de de çalışır; zenginleştirilmiş editörün web derlemesi upstream'de hâlâ deneysel olarak işaretlenmiştir.

### Widget'lar

SDK, FastComments Android SDK'sını yansıtan üç widget ile birlikte gelir:

- `FastCommentsLiveCommenting` - oylar, yanıtlar, sayfalama, bahsetmeler, bildirimler ve canlı güncellemeler içeren dallanmış yorumlama.
- `FastCommentsLiveChat` - aynı motor üzerinde bir sohbet ön ayarı: alt kısımda yeni mesajlar bulunan kronolojik mesajlar, listenin altında besteci, canlı başlık şeridi (bağlantı noktası + kullanıcı sayısı), yukarı kaydırarak yüklenen sonsuz geçmiş, yeni mesajlara otomatik kaydırma, oy veya yanıt dallanması yok. Her ön ayar `config` aracılığıyla geçersiz kılınabilir.
- `FastCommentsFeed` - gönderi bestecisi, medya, tepkiler, takipler ve canlı yeni-gönderi banner'ları içeren bir sosyal akış.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Tema

Varsayılan görünüm, bir dizi anlamsal tasarım token'ından (`FastCommentsTheme`) oluşturulur: renkler, boşluklar, yarıçap, yazı tipi boyutları, yazı tipi ağırlıkları ve avatar boyutları. Herhangi bir widget'ta `theme` prop'u aracılığıyla kısmi token geçersiz kılmaları (tipli `FastCommentsThemeOverrides`) iletin ve tüm stil ağacı tutarlı şekilde yeniden biçimlendirilir:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

Koyu mod yalnızca bir token seti uzaklıktadır:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

`styles` prop'u, hassas kontrol için ham bir `IFastCommentsStyles` ağacını hâlâ kabul eder. Hem `theme` hem de `styles` sağlandığında, açıkça belirtilen stiller temalı ağaçtan üstün gelir; yalnızca `styles` sağlandığında, varsayılanların yerini tamamen alır (orijinal davranış, böylece mevcut entegrasyonlar ve temalar etkilenmez). `setupDarkModeSkin`, `theme` prop'u lehine kullanımdan kaldırılmıştır.

### Yapılandırma Seçenekleri

Bu kütüphane, web uygulamasında olduğu gibi [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts) içinde tanımlanan tüm yapılandırma seçeneklerini desteklemeyi amaçlar.

Bunların üzerine, React Native `FastCommentsRNConfig` aracılığıyla birkaç SDK'ya özgü seçenek ekler:

- `hideTopBar` - bestecinin üzerinde gösterilen oturum açmış kullanıcı / bildirim zili şeridini gizler.
- `usePressToEdit` - menüsünü açmak için bir yoruma basılı tutun.
- `disableDownVoting` - aşağı oy düğmelerini gizler.
- `renderCommentInline` - yorumcu bilgilerini yorum içeriğiyle aynı HTML bloğunun içinde render eder.
- `renderLikesToRight` - oy/beğeni alanını yorumun altına değil sağına taşır.
- `renderDateBelowComment` - tarihi yorumun altında render eder.
- `showLiveStatus` - yorumların üzerinde sohbet tarzı "Canlı" + kullanıcı sayısı başlık şeridini gösterir.
- `useInlineSubmitButton` - gönder düğmesini bestecinin içinde bir simge olarak render eder.
- `countAboveToggle` - `useShowCommentsToggle` ile, "Yorumları Göster" düğmesinin üzerinde kaç yorumun render edileceği.
- `preserveFeedScrollPosition` - `FastCommentsFeed`, kaydırma konumunu unmount/remount işlemleri arasında hatırlar (varsayılan true).

### FastComments Kavramları

Başlamak için bilinmesi gereken ana kavramlar `tenantId` ve `urlId`'dir. `tenantId`, FastComments.com hesap tanımlayıcınızdır. `urlId`, yorum dizilerinin bağlanacağı yerdir. Bu bir sayfa URL'si, bir ürün kimliği, bir makale kimliği vb. olabilir.

### Yerelleştirme

Bu widget'lardaki kullanıcıya yönelik tüm metinler (düğme etiketleri, yer tutucular, boş durumlar, "5 dakika önce" gibi göreli tarihler, hata mesajları vb.) **sunucu tarafından yönetilir**. Bileşenler İngilizce dizeleri sabit kodlamaz; istenen yerel ayar için FastComments'in sunduğu çevirileri render eder.

Bir yerel ayar istemek için yapılandırmanızda `locale` değerini ayarlayın:

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, vb.
};
```

`locale` ayarlanmadığında, FastComments kiracının varsayılan dilini sunar.

**Metni düzenleme:** çeviriler bu SDK'da değil, FastComments panonuzda yönetilir. İfadeyi değiştirmek, varsayılan metni geçersiz kılmak veya bir dil eklemek için panodaki hesabınızın çevirilerini düzenleyin - değişiklik, uygulama sürümü yayınlamaya gerek kalmadan widget'lar tarafından otomatik olarak alınır. SDK İngilizce yedekler içermez, bu nedenle panoda boş bıraktığınız her anahtar boş render edilir; desteklediğiniz her yerel ayar için anahtarları doldurulmuş tutun.

### Kullanıcı Bildirimleri

FastComments, [birçok senaryo](https://docs.fastcomments.com/guide-notifications.html) için bildirimleri destekler. Bildirimler yapılandırılabilir, genel olarak veya bildirim/yorum düzeyinde devre dışı bırakılabilir ve kullanıcıların belirli bir sayfanın veya makalenin dizilerine abone olabilmesini sağlayan sayfa düzeyinde abonelikleri destekler.

Örneğin, kullanıcıyı kimlik doğrulamak için Secure SSO kullanmak ve ardından okunmamış bildirimleri düzenli olarak sorgulayıp kullanıcıya iletmek mümkündür.

Okunmamış kullanıcı bildirimlerini nasıl alacağınız ve çevireceğiniz için [AppNotificationSecureSSO örneğine](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) bakın.

### Gif Tarayıcısı

Varsayılan olarak, hiçbir görsel veya gif seçimi etkin değildir. Görsel ve gif yüklemelerini nasıl destekleyeceğiniz için [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) dosyasına bakın. Bu kütüphanede sağlanan, aramaları ve görselleri anonimleştiren bir Gif Tarayıcısı vardır; onu kullanmanız yeterlidir.

### Performans

Herhangi bir performans sorunu tespit ederseniz, kullanılan cihaz dahil, yeniden üretmek için bir örnek içeren bir bilet açın. Performans, tüm FastComments kütüphanelerinde birinci sınıf bir unsurdur.