Varsayılan olarak FastComments yorumlarda iframe'lere izin vermez. Medya gömmelerini etkinleştirdiğinizde, yorumcular YouTube, Vimeo, SoundCloud ve Spotify gibi güvenilir sağlayıcılardan gömme kodunu (`<iframe>` parçası) yapıştırabilir ve bu kod yorum içinde satır içi olarak görüntülenir.

Güvenlik nedeniyle, bu bir istemci tarafı widget yapılandırma bayrağı değildir. Bu, her yorum kaydedildiğinde doğrulanan sunucu tarafı bir ayardır, bu yüzden sayfadan etkinleştirilemez. Yalnızca yerleşik güvenilir sağlayıcılar listesine işaret eden iframe'lere izin verilir. Diğer tüm iframe'ler kaldırılır.

Bu, kod gerektirmeden widget özelleştirme sayfasında yapılır:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.allow-embeds'; selector = '.allow-embeds'; title='Allow Media Embeds' app-screenshot-end]

### Kendi Sağlayıcılarınızı Eklemek

Eğer yerleşik güvenilir listede olmayan bir sağlayıcıdan gömme izni vermek istiyorsanız, aynı sayfadaki "Additional Embed Domains" alanına sağlayıcının hostname'ini ekleyin. Bu hostnames, yerleşik sağlayıcılara ek olarak izinli sayılır. Eşleştirme tam eşleşmedir, bu yüzden tam host adını ekleyin (örneğin, player.example.com). Listelemediğiniz her şey engellenmeye devam eder.

Hem düz yorum kutusu hem de WYSIWYG düzenleyici gömme yapıştırmayı destekler. WYSIWYG düzenleyicide gömme, kaldırılabilir bir blok olarak eklenir.