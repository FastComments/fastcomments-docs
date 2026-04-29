Bütçe uyarı e-postaları, bir ajanın harcaması kotasının yapılandırılabilir bir yüzdesini aştığında tetiklenir. Faturadan sorumlu kişilere gönderilir.

### Uyarılar nasıl çalışır

Her ajanın düzenleme formunda bir **Alert thresholds** alanı vardır. Varsayılan olarak `80%` ve `100%`'dür. Bireysel eşiklerin işaretini koyup kaldırabilir ve başka yüzdeler ekleyebilirsiniz.

Bir ajanın belirli bir kapsamda (günlük veya aylık) harcaması bir dönemde ilk kez bir eşiği aştığında, platform her alıcıya bir e-posta gönderir. Aynı dönemde eşik daha sonra tekrar aşılırsa (ör., harcama 80%'in altına düşüp yeniden üstüne çıkarsa) tekrar gönderilmez.

Bu dönem başına işler: yeni bir günlük sıfırlama, o gün için eşik-aşma mantığını yeniden başlatır.

### Tenant kapsamlı uyarılar

Tenant (hesap) kendi günlük ve aylık kotalarına sahiptir. Tenant kapsamlı uyarılar sabit eşiklerde (`80%` ve `100%`) tetiklenir. Bunlar tüm tenant'a uygulandığı için ajana göre yapılandırılamaz.

### Alıcılar

Bütçe uyarıları şunlara gönderilir:

- Tenant üzerindeki **Super admin** olarak işaretlenmiş her kullanıcı.
- Tenant üzerindeki **Billing Admin** olarak işaretlenmiş her kullanıcı.

Bu, iki rolün birleşimini kapsar - her iki role sahip bir kullanıcı bir e-posta alır.

### Neden her iki rol

Super admins genellikle bir ajanın kotasına ulaştığını bilmesi gereken operasyonel kişilerdir. Billing admins faturanın sahibidir ve ajanları günlük olarak yönetip yönetmediklerine bakılmaksızın maliyet artışlarından haberdar olmaları gerekir. Ajanı gerçekten düzenleyebilmek (kotayı artırmak, duraklatmak) için alıcının ayrıca **Customization Admin** rolüne sahip olması gerekir - bu rol ajan düzenleme sayfasının erişimini kısıtlar.

### Kullanıcı bazlı vazgeçme

Profilinde yönetici bildirimlerinden vazgeçen alıcılar atlanır. Bu, diğer yönetici bildirimlerini kontrol eden aynı vazgeçme anahtarıdır.

Eğer **tüm** alıcılar vazgeçmişse, uyarı (uyarı düzeyinde) kaydedilir ve hiçbir e-posta gönderilmez.

### E-posta içeriği

E-postada şunlar yer alır:

- The **agent display name** and internal name.
- The **scope** that crossed (ör., "ajan günlük bütçesi", "ajan aylık bütçesi", "hesap günlük bütçesi", "hesap aylık bütçesi").
- The **threshold percentage** crossed.
- **Usage** tenant'ın para biriminde.
- **Cap** tenant'ın para biriminde.
- Tek tıklamayla imzalanmış bir oturum açma bağlantısı (one-click signed login link) alıcıyı doğrudan şuraya götürür:
  - Ajan kapsamlı uyarılar için ajan düzenleme sayfasına.
  - Tenant kapsamlı uyarılar için AI Agents list page.
  
Bağlantı önceden kimlik doğrulanmıştır, bu yüzden alıcı kotayı yükseltmek veya ajanı devre dışı bırakmak için tek tıklama uzaklıktadır.

### Eşikler nasıl tetiklenir

Platform, bu dönemde hangi eşiklerin zaten tetiklendiğini, ajan ve tenant için ayrı ayrı izler. Yani:

- Aynı dönemde önce 80% sonra 100%'ü geçmek, her ikisini sırayla tetikler.
- Bir kerede 0%'den 100%'e büyük bir sıçrama ile gitmek, aşılmış olan **en yüksek** eşiği (100%) tetikler, 80% değil; dolayısıyla en şiddetli uyarı gönderilir.

### Ne zaman uyarı almayı bırakırsınız

Eğer ajanın harcaması bu dönemde bir sonraki eşiğe hiç ulaşmazsa, bu dönemde başka e-postalar almazsınız. Bir sonraki günlük sıfırlama (veya aylık sıfırlama) izlemeyi temizler.

### Uyarıları devre dışı bırakma

İstemediğiniz eşkinin işaretini kaldırın. Belirli bir ajan için hiçbir uyarı almak istemiyorsanız, tüm yüzdelerin işaretini kaldırın. Tenant kapsamlı uyarılar ajana göre devre dışı bırakılamaz (bunlar tenant geneldir).

### Ayrıca bakınız

- [Budgets Overview](#budgets-overview).
- [Drop Reasons](#drop-reasons) - kotanın tamamen dolduğunda ne olur.
- [Cost Model](#cost-model) - neyin ölçüldüğü.

---