FastComments ile sitenizde yorum yapmayı engellemenin iki yolu vardır.

İlki, e-posta adreslerini zaten biliyorsanız, bunu <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">Yasaklı Kullanıcılar</a> sayfasına girebilirsiniz.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; title='The Banned Users Page' app-screenshot-end]

Bu sayfaya Yorumları Denetle -> Yasaklı Kullanıcılar yolunu izleyerek erişilebilir.

Bir kullanıcıyı yasaklarken tür seçebiliriz; Kalıcı veya Kalıcı Gölge Yasaklama:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; title='Banning a User' app-screenshot-end]

İkinci yol, Yorumları Denetleme sayfasındaki her yorumun üzerine yerleştirilen yasakla düğmesine tıklamaktır.

Yasakla düğmesine tıkladığınızda, yasak türünü ve süresini belirleyebileceğiniz bazı seçenekler gösterilir.

### E-posta Takma Adları

Bir kullanıcıyı e-posta ile yasaklarken, FastComments otomatik olarak `+` takma adlarını yok sayar. Örneğin, `user+alias@gmail.com` adresini yasaklamak, aynı zamanda `user@gmail.com` adresini ve `user+other@gmail.com` gibi o adresin diğer `+` varyasyonlarını da yasaklayacaktır.

### Gölge Yasaklamalar

Gölge yasaklama, kullanıcının yorumu veya oyu başarıyla kaydedilmiş gibi görünmesini sağlayan, ancak gerçekte kaydedilmediği bir yasaklama türüdür. Belirli durumlarda bu tercih edilebilir.

### IP Adresi ile Yasaklama

Bir tenant vazgeçmek istemedikçe, FastComments yorumcunun IP adresinin hashlenmiş bir versiyonunu saklayarak IP ile yasaklamayı destekler.

---