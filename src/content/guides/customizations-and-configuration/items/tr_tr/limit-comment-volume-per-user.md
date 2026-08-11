---
Varsayılan olarak, her kullanıcı aynı dakikada en fazla `5 yorum` gönderebilir.

Bu, kullanıcı kimliği, anonim kullanıcı kimliği ve IP adresi (hashlenmiş) ile izlenir.

Bu, kod yazmadan, widget özelleştirme sayfasında özelleştirilebilir:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; alt='Widget özelleştirme sayfasındaki dakikada maksimum yorum alanı, varsayılan olarak 5 olarak ayarlanmıştır'; title='Kullanıcı Başına Yorum Hacmini Sınırlama' app-screenshot-end]

Yorum oluşturma API'sini kullanıyorsanız, oran sınırlamasının kullanıcı başına uygulanması ve hesabınıza genel olarak uygulanmaması için istekte kullanıcının orijinal `ip` adresini arka uca göndermek isteyebilirsiniz.

---