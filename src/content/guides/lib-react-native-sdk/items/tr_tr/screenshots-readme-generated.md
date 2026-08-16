Live iş parçacıklı yorumlama, avatarlar, iç içe yanıtlar, oylar ve yerleşik zengin‑metin oluşturucu ile birlikte, ayrıca karanlık tema ve bir canlı‑sohbet ön ayarı (burada `react-native-web` ile render edilmiş olarak gösterilmiştir):

<table>
  <tr>
    <td align="center"><b>Canlı Yorumlama</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-light.png" width="260" alt="Canlı yorumlama, açık tema"/></td>
    <td align="center"><b>Karanlık Tema</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-dark.png" width="260" alt="Canlı yorumlama, karanlık tema"/></td>
    <td align="center"><b>Canlı Sohbet</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-chat.png" width="260" alt="Canlı sohbet ön ayarı"/></td>
  </tr>
</table>

### Zengin Metin Düzenleyici

Bu kütüphane, güçlü bir WYSIWYG düzenleme deneyimi sağlayan zengin metin düzenleme için [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) kullanır. Aynı düzenleyici iOS, Android ve web (`react-native-web` aracılığıyla) üzerinde çalışır, böylece oluşturucu tek bir uygulama ile her platformda tutarlı davranır.

`react-native-enriched`, yerel (RN 0.76'dan beri varsayılan, RN 0.72‑0.75'te isteğe bağlı) React Native Yeni Mimarisi (Fabric) ve paket `exports` koşullarını çözen bir paketleyici gerektirir. Bu SDK, RN 0.81 / React 19 ile geliştirilmiş ve test edilmiştir. Aynı düzenleyici ayrıca `react-native-web` üzerinden web'te çalışır; zenginleştirilmiş düzenleyicinin web yapısı hâlâ üst kaynakta deneysel olarak işaretlenmiştir.

### Widget'lar

SDK, FastComments Android SDK'sını yansıtan üç widget ile birlikte gelir:

- `FastCommentsLiveCommenting` - oylar, yanıtlar, sayfalama, bahsetmeler, bildirimler ve canlı güncellemelerle iş parçacıklı yorumlama.
- `FastCommentsLiveChat` - aynı motor üzerinde bir sohbet ön ayarı: yeni mesajların altta olduğu kronolojik mesajlar, listenin altında oluşturucu, canlı bir başlık çubuğu (bağlantı noktası + kullanıcı sayısı), yukarı kaydırarak yüklenecek sonsuz geçmiş, yeni mesajlara otomatik kaydırma, oylar veya yanıt iş parçacığı yok. Her ön ayar `config` aracılığıyla geçersiz kılınabilir.
- `FastCommentsFeed` - gönderi oluşturucu, medya, tepkiler, takipler ve canlı yeni gönderi banner'ları içeren bir sosyal akış.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Temalandırma

Varsayılan görünüm, bir dizi anlamsal tasarım belirteci (`FastCommentsTheme`) üzerinden oluşturulur: renkler, boşluklar, yarıçap, yazı tipi boyutları, yazı tipi ağırlıkları ve avatar boyutları. Herhangi bir widget'ta `theme` özelliği aracılığıyla kısmi belirteç geçersiz kılmalarını (`FastCommentsThemeOverrides` tipinde) geçirerek tüm stil ağacını tutarlı bir şekilde yeniden stilize edebilirsiniz:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

Karanlık mod bir belirteç seti uzakta:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

`styles` özelliği hâlâ cerrahi kontrol için ham bir `IFastCommentsStyles` ağacını kabul eder. `theme` ve `styles` birlikte sağlandığında, açıkça belirtilen stiller temalı ağacın üzerine geçer; yalnızca `styles` sağlandığında, varsayılanları tamamen değiştirir (orijinal davranış, böylece mevcut entegrasyonlar ve görünümler etkilenmez). `setupDarkModeSkin`, `theme` özelliği lehine kullanımdan kaldırılmıştır.

### Yapılandırma Seçenekleri

Bu kütüphane, web uygulaması gibi, [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts) içinde tanımlanan tüm yapılandırma seçeneklerini desteklemeyi amaçlar.

Bunların üzerine, React Native `FastCommentsRNConfig` aracılığıyla birkaç SDK'ya özgü seçenek ekler:

- `hideTopBar` - oluşturucunun üzerindeki oturum açmış kullanıcı / bildirim çanı çubuğunu gizler.
- `usePressToEdit` - bir yorumu basılı tutarak menüsünü açar.
- `disableDownVoting` - aşağı oy butonlarını gizler.
- `renderCommentInline` - yorumcu bilgisini yorum içeriğiyle aynı HTML bloğu içinde render eder.
- `renderLikesToRight` - oy/like alanını yorumun altı yerine sağ tarafına taşır.
- `renderDateBelowComment` - tarihi yorumun altına render eder.
- `showLiveStatus` - yorumların üzerindeki sohbet tarzı "Live" + kullanıcı sayısı başlık çubuğunu gösterir.
- `useInlineSubmitButton` - gönderim düğmesini oluşturucu içinde bir simge olarak render eder.
- `countAboveToggle` - `useShowCommentsToggle` ile, "Yorumları Göster" geçişinin üzerinde kaç yorumun render edileceğini belirler.
- `preserveFeedScrollPosition` - `FastCommentsFeed`, kaldırma/yeniden ekleme sırasında kaydırma ofsetini hatırlar (varsayılan true).

### FastComments Kavramları

Başlamak için bilmeniz gereken temel kavramlar `tenantId` ve `urlId`'dir. `tenantId`, FastComments.com hesabınızın tanımlayıcısıdır. `urlId`, yorum iş parçacıklarının bağlanacağı yerdir. Bu bir sayfa URL'si, bir ürün kimliği, bir makale kimliği vb. olabilir.

### Yerelleştirme

Bu widget'larda kullanıcıya gösterilen tüm metinler (düğme etiketleri, yer tutucular, boş durumlar, "5 dakika önce" gibi göreceli tarihler, hata mesajları vb.) **sunucu tarafından yönlendirilir**. Bileşenler İngilizce dizeleri sabit kodlamaz; istenen yerel ayar için FastComments tarafından sağlanan çevirileri render eder.

Bir yerel ayar talep etmek için, `locale`'i yapılandırmanızda ayarlayın:

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, etc.
};
```

`locale` ayarlanmamışsa, FastComments, kiracının varsayılan dilini sunar.

**Metni Düzenleme:** çeviriler FastComments kontrol panelinizde yönetilir, bu SDK içinde değil. Metni değiştirmek için varsayılan kopyayı geçersiz kılın veya bir dil ekleyin, kontrol panelinde hesabınız için çevirileri düzenleyin – değişiklik, uygulama sürümü gerektirmeden widget'lar tarafından otomatik olarak alınır. SDK, İngilizce geri dönüşler sağlamaz, bu yüzden kontrol panelinde boş bıraktığınız herhangi bir anahtar boş render edilir; desteklediğiniz her yerel ayar için anahtarları doldurmuş tutun.

### Kullanıcı Bildirimleri

FastComments, [birçok senaryo](https://docs.fastcomments.com/guide-notifications.html) için bildirimleri destekler. Bildirimler yapılandırılabilir, genel olarak veya bildirim/yorum düzeyinde devre dışı bırakılabilir ve sayfa düzeyinde abonelikleri destekler, böylece kullanıcılar belirli bir sayfa veya makale iş parçacıklarına abone olabilir.

Örneğin, kullanıcıyı kimlik doğrulamak için Secure SSO kullanmak ve ardından periyodik olarak okunmamış bildirimleri sorgulayıp kullanıcıya göndermek mümkündür.

Okunmamış kullanıcı bildirimlerini nasıl alıp çevireceğinizi görmek için [örnek AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) adresine bakın.

### Gif Tarayıcı

Varsayılan olarak, hiçbir resim veya gif seçimi etkin değildir. Resim ve gif yüklemelerini nasıl destekleyeceğinizi görmek için [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) adresine bakın. Bu kütüphanede sağlanan aramaları ve görüntüleri anonimleştiren bir Gif Tarayıcı vardır, sadece onu kullanmanız gerekir.

### Performans

Herhangi bir performans sorunu tespit ederseniz, kullanılan cihaz dahil olmak üzere yeniden üretmek için bir örnekle bir bilet açın. Performans, tüm FastComments kütüphanelerinin birinci sınıf bir özelliğidir.