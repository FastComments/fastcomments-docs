Ajan bir onay kuyruğa aldığında, platform gözden geçirenleri e-posta ile bilgilendirir. Düzenleme formundaki iki ayar bunu kontrol eder: **kimin** bilgilendirileceği ve **ne sıklıkta**.

### Kim: bildirim modu

İki mod:

- **Tüm yöneticiler ve moderatörler** (varsayılan) - tenant üzerindeki her hesap sahibi, süper yönetici ve yorum moderatörü yöneticisi aday gözden geçiren olur.
- **Belirli kullanıcılar** - düzenleme formundaki çift liste seçicisinden elle bir liste seçin.

Her iki durumda da, aday bir gözden geçirenin tenant üzerinde bir hesabı ve bildirim almak için geçerli bir e-posta adresi olmalıdır.

### Ne sıklıkta: kullanıcı başına sıklık

Her aday gözden geçirenin **kendi profili**, ajan onayları için kişisel bildirim sıklığını belirler:

- **Anında** (varsayılan) - bekleyen her onay için bir e-posta, onay oluşturulur oluşturulmaz gönderilir.
- **Saatlik** - o saat içinde kuyruğa alınan tüm onayları özetleyen saatte bir özet e-posta.
- **Günlük** - 24 saatte bir özet e-posta.
- **Devre dışı** - e-posta yok. Kullanıcı yine de gelen kutusu arayüzü üzerinden onayları inceleyebilir; sadece bilgilendirilmez.

Kullanıcı bu ayarı ajan düzenleme formunda değil, kendi profilinde değiştirir. Bu kasıtlıdır - bir tenant'ta on ajan olabilir ve bir moderatör, tercih ettiği sıklığı her ajan için ayrı ayrı ayarlamak zorunda olmamalıdır.

### Özet e-postaları yöneten cron işleri

- **`hourly-agent-approval-digest`** - her saat çalışır, her kullanıcının son özetinden bu yana kuyruğa alınan onayları gruplandırır, kullanıcı başına bir e-posta gönderir.
- **`daily-agent-approval-digest`** - aynısı, günlük.
- **`agent-approval-reaper`** - durumundan bağımsız olarak 90 günden eski onayları temizler.

Saatlik ve günlük özet cron'ları alıcı başına kapsamlıdır: saatlik sıklığa sahip bir kullanıcı saatlik cron tarafından işlenir ve günlük olan tarafından atlanır (ve tersi). Anında sıklığa sahip kullanıcılar, cron'lar tarafından değil, onay oluşturma kod yolu tarafından bilgilendirilir.

### Tekilleştirme durumu

Platform, her onay için hangi kullanıcılara zaten e-posta gönderildiğini takip eder. Bir kullanıcı bilgilendirildikten sonra (anında veya bir özet içinde), aynı onay için tekrar e-posta almaz - döngü ortasında sıklığını anında'dan günlük'e değiştirse bile.

### E-postadan onaylama

Her bildirim e-postası, gözden geçireni doğrudan onay detay sayfasına, zaten kimliği doğrulanmış şekilde götüren tek tıklamalı imzalı bir giriş bağlantısı içerir. Oradan onaylayabilir, reddedebilir veya [İstemleri İyileştir](#refining-prompts) akışını açabilir.

### Yönetici yoksa ne olur

`notifyMode` `All admins and moderators` ise ancak tenant'ta geçerli e-posta adresine sahip süper yönetici, yorum moderatörü yöneticisi veya hesap sahibi yoksa, platform bir uyarı kaydeder ve onay yine de kuyruğa alınır - sadece kimse bilgilendirilmez. Gelen kutusunda biri rastgele bakana kadar bekler.

`notifyMode` `Specific users` ise ancak hiçbir kullanıcı seçmediyseniz, sonuç aynıdır.

### Faturalandırma bildirimleri devre dışıysa ne olur

[Bütçe Uyarıları](#budget-alerts) - bütçe ile ilgili e-postalar - **kullanıcı başına bildirim tercihine bakılmaksızın** faturalandırma yöneticilerine gider. Bu kasıtlıdır: bütçe aşımı maliyeti etkiler ve faturalandırma sahibinin bunu bilmesi gerekir.

Onay bildirimleri yalnızca kullanıcı başına ajan onayı sıklığı ayarına uyar. Daha geniş yönetici-bildirimleri devre dışı bırakma seçeneğini kontrol etmezler - yönetici bildirimlerinden çıkmış bir kullanıcı, gözden geçiren listesindeyse yine de onay e-postaları alır, ajan onayı sıklığı **Devre dışı** olarak ayarlanmadığı sürece.

### Ayrıca bakınız

- Bir onayın tam yaşam döngüsü için [Onay İş Akışı](#approval-workflow).
- "Aynı türdeki hatayı sürekli onaylıyorum" iş akışı için [İstemleri İyileştirme](#refining-prompts).