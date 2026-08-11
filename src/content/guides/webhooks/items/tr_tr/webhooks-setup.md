---
Üretimde yaptığınız gibi `localhost` için aynı adımları izleyin. Üretim alan adlarınızın ve API Gizli Anahtarlarınızın ayarlandığından emin olun.

İlk olarak, [Webhooks admin](https://fastcomments.com/auth/my-account/manage-data/webhooks) adresine gidin. Bu, Manage Data -> Webhooks üzerinden erişilebilir.

Yapılandırma sayfası aşağıdaki gibi görünür:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; alt='Yorum etkinliği başına bir alan adı seçici ve bir uç nokta URL alanı içeren Webhooks yönetim sayfası, ayrıca Test Yükü Gönder'; title='Webhooks Yapılandırması'; cacheBuster = 'v3' app-screenshot-end]

Bu sayfada, her yorum etkinliği türü için uç noktaları belirtebilirsiniz.

Her etkinlik türü için, entegrasyonunuzu doğru şekilde kurduğunuzdan emin olmak amacıyla Test Yükü Gönder düğmesine tıkladığınızdan emin olun. Ayrıntılar için bir sonraki bölüm olan "Testing" kısmına bakın.

---