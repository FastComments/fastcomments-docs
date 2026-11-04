Bir ajanın harcaması, yapılandırılabilir bir yüzdede sınırını aştığında bütçe uyarı e-postaları tetiklenir. Bu e-postalar, faturayı ödeyen kişilere gider.

### Uyarılar nasıl çalışır

Her ajanın düzenleme formunda bir **Uyarı eşikleri** alanı bulunur. Varsayılan olarak `80%` ve `100%` değerleridir. Tek tek eşikleri işaretleyebilir veya işaretini kaldırabilir ve başka yüzdeler ekleyebilirsiniz.

Belirli bir kapsamda (günlük veya aylık) ajanın harcaması, o dönem içinde ilk kez bir eşiği aştığında, platform alıcı başına bir e-posta gönderir. Aynı dönem içinde eşiğin daha sonra tekrar aşılması (örneğin, harcama %80'in altına düşüp tekrar üzerine çıkması) e-postayı **tekrar göndermez**.

Bu dönem bazındadır: yeni bir günlük sıfırlama, o gün için eşik aşma mantığını yeniden başlatır.

### Kiracı kapsamlı uyarılar

Kiracının (hesabın) kendi günlük ve aylık sınırları vardır. Kiracı kapsamlı uyarılar sabit eşiklerde tetiklenir (`80%` ve `100%`). Bunlar tüm kiracıya uygulandığı için ajan başına yapılandırılamaz.

### Alıcılar

Bütçe uyarıları şunlara gönderilir:

- Kiracıda **Super admin** olarak işaretlenmiş her kullanıcı.
- Kiracıda **Billing Admin** olarak işaretlenmiş her kullanıcı.

Bu, iki rolün birleşimini içerir - her iki role de sahip bir kullanıcı tek bir e-posta alır.

### Neden her iki rol

Super admin'ler genellikle bir ajanın sınırına ulaştığını bilmesi gereken operatörlerdir. Faturalandırma yöneticileri faturanın sahibidir ve ajanları gün içinde yönetip yönetmediklerinden bağımsız olarak maliyet artışlarını bilmeleri gerekir. Ajanı gerçekten düzenlemek için (sınırı yükseltmek, duraklatmak), alıcının ayrıca ajan düzenleme sayfasını kontrol eden **Customization Admin** rolüne de sahip olması gerekir.

### Kullanıcı bazlı çıkış

Profillerinde yönetici bildirimlerinden çıkmış olan alıcılar atlanır. Bu, diğer yönetici bildirimlerini kontrol eden aynı çıkış anahtarıdır.

**Tüm** alıcılar çıkış yapmışsa, uyarı günlüğe kaydedilir (uyarı seviyesi) ve e-posta gönderilmez.

### E-posta içeriği

E-posta şunları içerir:

- **Ajan görünen adı** ve dahili adı.
- Aşılan **kapsam** (örneğin, "ajan günlük bütçesi", "ajan aylık bütçesi", "hesap günlük bütçesi", "hesap aylık bütçesi").
- Aşılan **eşik yüzdesi**.
- Kiracının para biriminde **Kullanım**.
- Kiracının para biriminde **Sınır**.
- Alıcıyı doğrudan şuraya götüren **tek tıklamalı imzalı giriş bağlantısı**:
  - Ajan kapsamlı uyarılar için ajan düzenleme sayfası.
  - Kiracı kapsamlı uyarılar için AI Agents liste sayfası.

Bağlantı önceden kimlik doğrulamalıdır, böylece alıcı sınırı yükseltmek veya ajanı devre dışı bırakmak için tek tıklama uzaklıktadır.

### Eşikler nasıl tetiklenir

Platform, bu dönem içinde hangi eşiklerin zaten tetiklendiğini, ajan ve kiracı için ayrı ayrı takip eder. Yani:

- Aynı dönemde önce %80'i sonra %100'ü aşmak her ikisini de sırayla tetikler.
- Tek bir büyük sıçramayla doğrudan %0'dan %100'e gitmek, %80'i değil, aşılan **en yüksek** eşiği (%100) tetikler, böylece en ciddi uyarı teslim edilen olur.

### Uyarı almayı ne zaman bırakırsınız

Ajanın harcaması bu dönemde bir sonraki eşiğe ulaşmazsa, bu dönemde başka e-posta almazsınız. Bir sonraki günlük sıfırlama (veya aylık sıfırlama) takibi temizler.

### Uyarıları devre dışı bırakma

İstemediğiniz eşiğin işaretini kaldırın. Belirli bir ajan için hiçbir uyarı istemiyorsanız, tüm yüzdelerin işaretini kaldırın. Kiracı kapsamlı uyarılar ajan bazında devre dışı bırakılamaz (kiracı genelinizedir).

### Ayrıca bakınız

- [Budgets Overview](#budgets-overview).
- [Drop Reasons](#drop-reasons) - sınıra tamamen ulaşıldığında ne olur.
- [Cost Model](#cost-model) - neyin ölçüldüğü.