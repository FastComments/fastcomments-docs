Varsayılan olarak FastComments yorumlarda iframe'lere izin vermez. Medya gömmelerini etkinleştirdiğinizde, yorumcular YouTube, Vimeo, SoundCloud ve Spotify gibi güvenilir sağlayıcılardan gömme kodunu ( `<iframe>` snippet'ini) yapıştırabilir ve bu yorum içinde satır içi olarak görüntülenir.

Güvenlik nedeniyle, bu bir istemci tarafı widget yapılandırma bayrağı değildir. Bu, her yorum kaydedildiğinde doğrulanan bir sunucu tarafı ayardır, bu yüzden sayfadan açılıp kapatılamaz. Yalnızca yerleşik güvenilir sağlayıcılar listesine işaret eden iframe'lere izin verilir. Diğer tüm iframe'ler kaldırılır.

Bu, kod yazmadan, widget özelleştirme sayfasında yapılır:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.allow-embeds'; selector = '.allow-embeds'; alt='Widget özelleştirme sayfasında medya gömme ayarı etkinleştirildi, yorumcuların güvenilir iframe gömmelerini yapıştırmasına izin veriyor'; title='Medya Gömme İzin Ver' app-screenshot-end]

### Kendi Sağlayıcılarınızı Eklemek

Yerleşik güvenilir listede bulunmayan bir sağlayıcıdan gömmelere izin vermek istiyorsanız, aynı sayfadaki "Additional Embed Domains" alanına sağlayıcının ana bilgisayar adını ekleyin. Bu ana bilgisayar adları yerleşik sağlayıcıların yanı sıra da izin verilir. Eşleşme tamdır, bu yüzden tam ana bilgisayar adını (örneğin, player.example.com) ekleyin. Listede yer almayan her şey engellenir.

Hem düz yorum kutusu hem de WYSIWYG editörü gömme yapıştırmayı destekler. WYSIWYG editöründe gömme, kaldırılabilir bir blok olarak eklenir.