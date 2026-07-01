Live threaded commenting with avatars, nested replies, votes, and the built-in rich-text composer, plus a dark theme and a live-chat preset (shown here rendered via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Canlı Yorumlama</b><br/><img src="./demo-screenshots/light.png" width="260" alt="Canlı yorumlama, açık tema"/></td>
    <td align="center"><b>Karanlık Tema</b><br/><img src="./demo-screenshots/dark.png" width="260" alt="Canlı yorumlama, karanlık tema"/></td>
    <td align="center"><b>Canlı Sohbet</b><br/><img src="./demo-screenshots/chat.png" width="260" alt="Canlı sohbet ön ayarı"/></td>
  </tr>
</table>

### Zengin Metin Düzenleyici

Bu kütüphane, zengin metin düzenleme için [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) kullanır; bu, güçlü bir WYSIWYG düzenleme deneyimi sağlar. Aynı düzenleyici iOS, Android ve web (via `react-native-web`) platformlarını güçlendirir, bu sayede besteci (composer) tek bir uygulama ile tüm platformlarda tutarlı davranır.

`react-native-enriched` yerel (native) ortamda React Native Yeni Mimarisi (Fabric) gerektirir (RN 0.76'dan beri varsayılan, RN 0.72‑0.75'te isteğe bağlı) ve paket `exports` koşullarını çözen bir paketleyici gerekir. Bu SDK, RN 0.81 / React 19 üzerinde geliştirilmiş ve test edilmiştir. Aynı düzenleyici ayrıca `react-native-web` üzerinden web'de çalışır; zengin düzenleyicinin web yapısı hâlâ üst kaynakta deneysel olarak işaretlenmiştir.

### Widget'lar

SDK, FastComments Android SDK'sını yansıtan üç widget gönderir:

- `FastCommentsLiveCommenting` – oylar, yanıtlar, sayfalama, bahsetmeler, bildirimler ve canlı güncellemelerle mesajlı yorumlama.
- `FastCommentsLiveChat` – aynı motor üzerine kurulmuş bir sohbet ön ayarı: alt kısımda yeni mesajların göründüğü kronolojik mesaj listesi, besteci (composer) listenin altında, canlı başlık çubuğu (bağlantı noktası + kullanıcı sayısı), yukarı kaydırarak yüklenecek sonsuz tarihçe, yeni mesajlara otomatik kaydırma, oy ya da yanıt zinciri yoktur. Her ön ayar `config` aracılığıyla geçersiz kılınabilir.
- `FastCommentsFeed` – gönderi bestecisi, medya, tepkiler, takipler ve yeni gönderi banner'larıyla sosyal akış.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Temalandırma

Varsayılan görünüm, bir dizi anlamsal tasarım token'ı (`FastCommentsTheme`) üzerinden üretilir: renkler, boşluklar, yarıçap, yazı tipi boyutları, yazı tipi ağırlıkları ve avatar boyutları. Herhangi bir widget'ta `theme` prop'u aracılığıyla kısmi token geçersiz kılmaları (type `FastCommentsThemeOverrides`) gönderin; tüm stil ağacı tutarlı bir şekilde yeniden stil alır:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

Karanlık mod sadece bir token seti uzakta:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

`styles` prop'u hâlâ cerrahi kontrol için ham `IFastCommentsStyles` ağacını kabul eder. `theme` ve `styles` ikisi de sağlanırsa, açıkça tanımlanmış stiller temalı ağaçtan üstün gelür; yalnızca `styles` sağlanırsa, varsayılanlar tamamen onunla değiştirilir (orijinal davranış, böylece mevcut entegrasyonlar ve skin'ler etkilenmez). `setupDarkModeSkin` artık `theme` prop'u yerine geçmesi nedeniyle kullanım dışı bırakılmıştır.

### Yapılandırma Seçenekleri

Bu kütüphane, web implementasyonu gibi, [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts) içinde tanımlanan tüm yapılandırma seçeneklerini desteklemeyi hedefler.

Bunların üzerine, React Native, `FastCommentsRNConfig` aracılığıyla birkaç SDK‑özgü seçenek ekler:

- `hideTopBar` – bestecinin üzerindeki oturum açmış kullanıcı / bildirim-zil çubuğunu gizler.
- `usePressToEdit` – bir yorumu uzun basarak menüsünü açar.
- `disableDownVoting` – aşağı oy butonlarını gizler.
- `renderCommentInline` – yorumcu bilgisini yorum içeriğiyle aynı HTML bloğu içinde render eder.
- `renderLikesToRight` – oy/like alanını yorumun altı yerine sağ tarafına taşır.
- `renderDateBelowComment` – tarihi yorumun altına render eder.
- `showLiveStatus` – yorumların üzerindeki sohbet‑stilinde "Live" + kullanıcı‑sayısı başlık çubuğunu gösterir.
- `useInlineSubmitButton` – gönderim düğmesini besteci içinde bir ikon olarak render eder.
- `countAboveToggle` – `useShowCommentsToggle` ile birlikte, "Show Comments" geçişinin üstünde kaç yorumun render edileceğini belirtir.
- `preserveFeedScrollPosition` – `FastCommentsFeed`, montaj/kaldırma (unmount/remount) arasında kaydırma ofsetini hatırlar (varsayılan true).

### FastComments Kavramları

Başlamak için bilmeniz gereken temel kavramlar `tenantId` ve `urlId`'dir. `tenantId`, FastComments.com hesabınızın tanımlayıcısıdır. `urlId` ise yorum dizilerinin bağlanacağı yerdir. Bu bir sayfa URL'si, bir ürün kimliği, bir makale kimliği vb. olabilir.

### Yerelleştirme

Bu widget'lardaki tüm kullanıcıya yönelik metinler (düğme etiketleri, yer tutucular, boş durumlar, "5 minutes ago" gibi göreceli tarih ifadeleri, hata mesajları vb.) **sunucu‑tahrikli**'dir. Bileşenler İngilizce dizeleri sabit kodlamaz; istenen yerel ayar için FastComments tarafından sunulan çevirileri render eder.

Yerel ayar (locale) talep etmek için `config` içinde `locale` ayarlayın:

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, etc.
};
```

`locale` ayarlanmamışsa, FastComments tenant'ın varsayılan dilini sunar.

**Metni düzenleme:** çeviriler FastComments panonuzda yönetilir, bu SDK içinde değildir. Metni değiştirmek, varsayılan kopyayı geçersiz kılmak veya yeni bir dil eklemek için panodaki çevirileri düzenin – değişiklik, uygulama yayınına ihtiyaç duymadan widget'lar tarafından otomatik olarak alınır. SDK, İngilizce geri dönüşleri sağlamaz; panoda boş bıraktığınız herhangi bir anahtar boş render edilir; desteklediğiniz her yerel ayar için anahtarların dolu olduğundan emin olun.

### Kullanıcı Bildirimleri

FastComments, [birçok senaryo](https://docs.fastcomments.com/guide-notifications.html) için bildirimleri destekler. Bildirimler yapılandırılabilir, küresel ya da bildirim/yorum seviyesinde devre dışı bırakılabilir ve sayfa‑seviyesi abonelikleri destekler; böylece kullanıcılar belirli bir sayfa ya da makale dizisine abone olabilir.

Örneğin, Secure SSO kullanarak kullanıcıyı kimlik doğrulamak ve ardından periyodik olarak okunmamış bildirimleri sorgulayıp kullanıcıya itmek mümkündür.

Okunmamış kullanıcı bildirimlerini nasıl alıp çevireceğinize dair örnek uygulamayı inceleyin: [AppNotificationSecureSSO örneği](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx).

### Gif Tarayıcısı

Varsayılan olarak, hiçbir resim ya da gif seçimi etkin değildir. Resim ve gif yüklemelerini nasıl destekleyeceğinize dair örneği inceleyin: [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx). Bu kütüphanede anonimleştirilmiş aramaları ve sağlanan görselleri içeren bir Gif Tarayıcısı vardır; sadece onu kullanmanız gerekir.

### Performans

Herhangi bir performans sorunu tespit ederseniz, kullanılan cihaz da dahil olmak üzere bir örnekle birlikte bir bilet açın. Performans, tüm FastComments kütüphanelerinin birinci sınıf bir vatandaşıdır.