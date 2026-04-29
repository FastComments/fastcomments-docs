When the agent queues an approval, the platform notifies reviewers via email. Two settings on the edit form control this: **who** is notified and **how often**.

### Kim: bildirim modu

İki mod:

- **Tüm yöneticiler ve moderatörler** (varsayılan) - tenant üzerindeki her hesap sahibi, süper yönetici ve yorum moderatörü yöneticisi aday inceleyicidir.
- **Belirli kullanıcılar** - düzenleme formundaki çift listeli seçiciden bir listeyi elle seçin.

Her iki durumda da, aday bir inceleyicinin bildirim alabilmesi için tenant üzerinde bir hesabı ve geçerli bir e-posta adresi olmalıdır.

### Ne sıklıkla: kullanıcı başına bildirim sıklığı

Her aday inceleyicinin **kendi profili**, agent onayları için kişisel bildirim sıklığını belirler:

- **Anında** (varsayılan) - bekleyen her onay için bir e-posta; onay oluşturulur oluşturulmaz gönderilir.
- **Saatlik** - o saatte sıraya alınan tüm onayları özetleyen saatlik bir özet e-postası.
- **Günlük** - her 24 saatte bir özet e-postası.
- **Devre Dışı** - e-posta gönderilmez. Kullanıcı yine de onayları gelen kutusu arayüzünden inceleyebilir; sadece bildirim gönderilmez.

Kullanıcı bu ayarı kendi profili üzerinde değiştirir, agent düzenleme formu üzerinde değil. Bu kasıtlıdır — bir tenant'ın on agent'ı olabilir ve bir moderatörün tercih ettiği sıklığı her agent için ayrı ayrı ayarlaması gerekmez.

### Özetleri yöneten cron işleri

- **`hourly-agent-approval-digest`** - her saat çalışır, her kullanıcının son özetinden bu yana sıraya alınan onayları gruplayıp her kullanıcıya bir e-posta gönderir.
- **`daily-agent-approval-digest`** - aynı şekilde, günlük.
- **`agent-approval-reaper`** - durumuna bakılmaksızın 90 günden eski onayları temizler.

Saatlik ve günlük özet cron'ları alıcı başına sınırlandırılmıştır: saatlik sıklığa sahip bir kullanıcı saatlik cron tarafından işlenir ve günlük cron tarafından atlanır (ve tersi). Anında sıklığındaki kullanıcılar cronlar tarafından değil, approval-create kod yolu tarafından bilgilendirilir.

### Tekilleştirme durumu

Platform, her onay hakkında hangi kullanıcılara zaten e-posta gönderildiğini takip eder. Bir kullanıcı bilgilendirildiğinde (anında veya özet içinde), aynı onay için tekrar e-posta gönderilmeyecektir — hatta döngü ortasında sıklığını anından günlük'e değiştirse bile.

### E-posta üzerinden onaylama

Her bildirim e-postası, inceleyiciyi zaten kimlik doğrulanmış halde onay detay sayfasına götüren tek tıklamalık imzalı bir giriş bağlantısı içerir. Oradan onaylayabilir, reddedebilir veya [İstemleri İyileştir](#refining-prompts) akışını açabilirler.

### Yönetici yoksa ne olur

Eğer `notifyMode` `All admins and moderators` ise fakat tenant'ta geçerli e-posta adresine sahip süper yöneticiler, yorum moderatörü yöneticileri veya hesap sahipleri yoksa, platform bir uyarı kaydeder ve onay yine de sıraya alınır — sadece kimse bunun hakkında bilgilendirilmez. Onay, birisi bakana kadar gelen kutusunda bekler.

Eğer `notifyMode` `Specific users` ise ama hiç kullanıcı seçmediyseniz, sonuç aynıdır.

### Faturalama bildirimleri devre dışıysa ne olur

[Bütçe Uyarıları](#budget-alerts) - bütçeyle ilgili e-postalar - faturalama yöneticilerine **kullanıcıya özel bildirim tercihinden bağımsız olarak** gider. Bu kasıtlıdır: bütçe aşımı maliyeti etkiler ve faturalama sahibinin haberdar olması gerekir.

Onay bildirimleri yalnızca kullanıcıya özel agent-onay sıklığı ayarına uyar. Bunlar geniş kapsamlı yönetici-bildirimlerinden vazgeçme ayarını kontrol etmez — yönetici bildirimlerinden vazgeçmiş bir kullanıcı, eğer inceleyici listesinde yer alıyorsa ve agent-onay sıklığı **Devre Dışı** değilse hâlâ onay e-postaları alır.

### Ayrıca bakınız

- [Onay İş Akışı](#approval-workflow) bir onayın tam yaşam döngüsü için.
- [İstemleri İyileştir](#refining-prompts) "Aynı tür hatayı sürekli onaylıyorum" iş akışı için.