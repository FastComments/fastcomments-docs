Live threaded commenting with avatars, nested replies, votes, and the built-in rich-text composer, plus a dark theme and a live-chat preset (shown here rendered via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Canlı Yorumlama</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-light.png" width="260" alt="Canlı yorumlama, açık tema"/></td>
    <td align="center"><b>Koyu Tema</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-dark.png" width="260" alt="Canlı yorumlama, koyu tema"/></td>
    <td align="center"><b>Canlı Sohbet</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-chat.png" width="260" alt="Canlı sohbet ön ayarı"/></td>
  </tr>
</table>

### Rich Text Editor

Bu kütüphane, zengin metin düzenleme için [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched-html) kullanır ve güçlü bir WYSIWYG düzenleme deneyimi sunar. Aynı düzenleyici iOS, Android ve web ( `react-native-web` aracılığıyla) üzerinde çalışır, bu sayede düzenleyici tek bir uygulama ile her platformda tutarlı davranır.

`react-native-enriched` native üzerinde React Native Yeni Mimarisi (Fabric) gerektirir (RN 0.76'dan beri varsayılan, RN 0.72-0.75'te isteğe bağlı), ve paket `exports` koşullarını çözen bir bundler gerekir. Bu SDK RN 0.81 / React 19 ile geliştirilmiş ve test edilmiştir. Aynı düzenleyici `react-native-web` üzerinden web'de de çalışır; zenginleştirilmiş düzenleyicinin web yapısı hâlâ üst kaynakta deneysel olarak işaretlenmiştir.

### Widgets

SDK, FastComments Android SDK'sını yansıtan üç widget ile birlikte gelir:

- `FastCommentsLiveCommenting` - oylar, yanıtlar, sayfalama, bahsetmeler, bildirimler ve canlı güncellemelerle zincirli yorumlama.
- `FastCommentsLiveChat` - aynı motor üzerinde bir sohbet ön ayarı: yeni mesajlar altta olacak şekilde kronolojik mesajlar, listeden sonra düzenleyici, canlı bir başlık çubuğu (bağlantı noktası + kullanıcı sayısı), yukarı kaydırarak yüklenecek sonsuz geçmiş, yeni mesajlara otomatik kaydırma, oylar veya yanıt zincirleme yok. Her ön ayar `config` aracılığıyla geçersiz kılınabilir.
- `FastCommentsFeed` - gönderi düzenleyicisi, medya, tepkiler, takipler ve yeni gönderi canlı bannerları içeren bir sosyal akış.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Theming

Varsayılan görünüm, bir dizi anlamsal tasarım token'ı (`FastCommentsTheme`) üzerinden oluşturulur: renkler, boşluklar, yarıçap, yazı tipi boyutları, yazı tipi ağırlıkları ve avatar boyutları. Herhangi bir widget'ta `theme` prop'u aracılığıyla kısmi token geçersiz kılmalarını (`FastCommentsThemeOverrides` tipinde) geçerek tüm stil ağacını tutarlı bir şekilde yeniden stilize edebilirsiniz:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

Karanlık mod bir token seti uzakta:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

`styles` prop'u hâlâ cerrahi kontrol için ham bir `IFastCommentsStyles` ağacını kabul eder. `theme` ve `styles` birlikte sağlandığında, açık stiller temalı ağacın üzerine geçer; yalnızca `styles` sağlandığında, varsayılanları tamamen değiştirir (orijinal davranış, böylece mevcut entegrasyonlar ve görünümler etkilenmez). `setupDarkModeSkin` `theme` prop'u lehine kullanımdan kaldırılmıştır.

### Configuration Options

Bu kütüphane, web uygulaması gibi, [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts) içinde tanımlanan tüm yapılandırma seçeneklerini desteklemeyi amaçlar.

Bu seçeneklerin üzerine, React Native `FastCommentsRNConfig` aracılığıyla birkaç SDK'ya özgü seçenek ekler:

- `hideTopBar` - düzenleyicinin üzerindeki oturum açmış kullanıcı / bildirim çanı çubuğunu gizler.
- `usePressToEdit` - bir yorumu basılı tutarak menüsünü açar.
- `disableDownVoting` - aşağı oy butonlarını gizler.
- `renderCommentInline` - yorumcu bilgisini yorum içeriğiyle aynı HTML bloğu içinde render eder.
- `renderLikesToRight` - oy/like alanını yorumun altı yerine sağ tarafına taşır.
- `renderDateBelowComment` - tarihi yorumun altına render eder.
- `showLiveStatus` - yorumların üzerindeki sohbet tarzı "Live" + kullanıcı sayısı başlık çubuğunu gösterir.
- `useInlineSubmitButton` - gönder butonunu düzenleyici içinde bir simge olarak render eder.
- `countAboveToggle` - `useShowCommentsToggle` ile, "Yorumları Göster" geçişinin üzerinde kaç yorumun render edileceği.
- `preserveFeedScrollPosition` - `FastCommentsFeed`, kaldırma/yeniden ekleme sırasında kaydırma ofsetini hatırlar (varsayılan true).

### FastComments Concepts

Başlamak için bilmeniz gereken temel kavramlar `tenantId` ve `urlId`'dir. `tenantId`, FastComments.com hesabınızın tanımlayıcısıdır. `urlId`, yorum dizilerinin bağlanacağı yerdir. Bu bir sayfa URL'si, bir ürün kimliği, bir makale kimliği vb. olabilir.

### Localization

Bu widget'larda (düğme etiketleri, yer tutucular, boş durumlar, "5 dakika önce" gibi göreceli tarihler, hata mesajları vb.) görünen tüm metin **sunucu tarafından yönlendirilir**. Bileşenler İngilizce dizeleri sabit kodlamaz; istenen yerel ayar için FastComments tarafından sağlanan çevirileri render eder.

Bir yerel ayar talep etmek için, yapılandırmanızda `locale` ayarlayın:

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, etc.
};
```

`locale` ayarlanmamışsa, FastComments kiracının varsayılan dilini sunar.

**Metni düzenleme:** çeviriler FastComments kontrol panelinizde yönetilir, bu SDK'da değil. Metni değiştirmek, varsayılan kopyayı geçersiz kılmak veya bir dil eklemek için, kontrol panelinde hesabınız için çevirileri düzenleyin - değişiklik widget'lar tarafından otomatik olarak alınır ve uygulama sürümü gerektirmez. SDK İngilizce geri dönüşler sağlamaz, bu yüzden kontrol panelinde boş bıraktığınız herhangi bir anahtar boş render edilir; desteklediğiniz her yerel ayar için anahtarları doldurmuş tutun.

### User Notifications

FastComments, [birçok senaryo](https://docs.fastcomments.com/guide-notifications.html) için bildirimleri destekler. Bildirimler yapılandırılabilir, genel olarak veya bir bildirim/yorum düzeyinde devre dışı bırakılabilir ve sayfa düzeyinde abonelikleri destekler, böylece kullanıcılar belirli bir sayfa veya makale dizisine abone olabilir.

Örneğin, kullanıcıyı kimlik doğrulamak için Secure SSO kullanmak ve ardından periyodik olarak okunmamış bildirimleri sorgulayıp kullanıcıya itmek mümkündür.

Nasıl okunmamış kullanıcı bildirimlerini alıp çevireceğinizi görmek için [örnek AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) inceleyin.

### Gif Browser

Varsayılan olarak, hiçbir resim veya gif seçimi etkin değildir. Resim ve gif yüklemelerini nasıl destekleyeceğinizi görmek için [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) inceleyin. Bu kütüphanede sağlanan aramaları ve görüntüleri anonimleştiren bir Gif Tarayıcısı vardır, sadece onu kullanmanız yeterlidir.

### Performance

Herhangi bir performans sorunu tespit ederseniz, kullanılan cihaz dahil olmak üzere yeniden üretmek için bir örnekle bir bilet açın. Performans, tüm FastComments kütüphanelerinin birinci sınıf bir özelliğidir.