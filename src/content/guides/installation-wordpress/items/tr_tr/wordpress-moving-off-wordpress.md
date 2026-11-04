Sitenizi WordPress'ten taşıyorsanız ve yeni sitenizde FastComments kullanmak istiyorsanız, WordPress eklentisine ihtiyacınız yoktur. Yorumlarınızı WordPress'ten dışa aktarın, ardından dosyayı FastComments panelindeki [İçe Aktarma sayfasına](https://fastcomments.com/auth/my-account/manage-data/import) yükleyin.

İki WordPress dışa aktarma formatını destekliyoruz.

### WordPress XML (Önerilen)

Bu, WordPress'in yerleşik dışa aktarıcısından gelen dosyadır, bu nedenle ek bir eklentiye gerek yoktur.

1. WordPress yönetim panelinizde `Tools -> Export` bölümüne gidin.
2. `All content` seçeneğini seçin ve `Download Export File` düğmesine tıklayın.
3. FastComments [İçe Aktarma sayfasında](https://fastcomments.com/auth/my-account/manage-data/import), `WordPress (.xml)` seçeneğini seçin ve dosyayı yükleyin.

Her yorum, bırakıldığı yazının URL'sine bağlıdır ve bu URL dosyada zaten mevcuttur.

İçe aktarma; yazar adını, e-postayı ve web sitesini, tarihi, içeriği, yanıt dizilerini ve yorumun onaylanıp onaylanmadığını korur. Yorumcu avatarları Gravatar'dan taşınır. Oylar bu formatın bir parçası değildir.

### WordPress CSV

Bu, [WebToffee'nin WordPress Comments Import & Export eklentisinden](https://wordpress.org/plugins/comments-import-export-woocommerce/) gelen dosyadır.

1. Eklentiyi WordPress yönetim panelinize yükleyin ve yorumlarınızı CSV olarak dışa aktarın.
2. Her `comment_post_ID` değerini yazının URL'si ile değiştirin.
3. FastComments [İçe Aktarma sayfasında](https://fastcomments.com/auth/my-account/manage-data/import), `WordPress (.csv)` seçeneğini seçin ve dosyayı yükleyin.

Her yorum `comment_post_ID` sütununa bağlıdır. WordPress bu sütunu yazı kimliği (post ID) ile doldurur ve yeni sitenizde WordPress yazı kimlikleri bulunmadığından, 2. adım bunu URL ile değiştirir.

İçe aktarma; yazar adını, e-postayı ve web sitesini, tarihi, içeriği, yanıt dizilerini ve yorumun onaylanıp onaylanmadığını korur. Yorumcu avatarları Gravatar'dan taşınır. Ayrıca dosyada bulunması durumunda WordPress'in spam işaretini ve wpDiscuz beğeni/beğenmeylerini de korur.

### Yorumları Yeni Sayfalarınızla Eşleştirme

Yeni siteniz WordPress sitenizle aynı URL'leri koruyorsa, yorumlar ek bir kurulum yapmadan eşleşen sayfalarda görünür.

Alan adı değişirse, içe aktarmadan sonra [Alan Adı Taşıma aracını](/guide-migrations.html#migrating-domains) çalıştırın. Tek tek sayfa URL'leri değişirse, her sayfayı eski URL'sinden yeni URL'sine [taşıyabilirsiniz](/guide-migrations.html#migrating-pages).

Yorum widget'ının [urlId](/guide-customizations-and-configuration.html#url-id) alanına ilettiğiniz değerden alan adını kaldırmak gibi toplu sayfa taşımaları için [bir destek talebi açın](https://fastcomments.com/auth/my-account/help), biz sizin için hallederiz.

### Geçiş Yapmadan Önce

İçe aktarmayı istediğiniz kadar çalıştırabilirsiniz. Aynı dosyayı yeniden içe aktarmak [yinelenen kayıtlar oluşturmaz](/guide-migrations.html#importing-data), böylece yeni siteyi test etmek için bir kez içe aktarabilir, ardından geçiş yapmadan hemen önce en son yorumlarınızla tekrar içe aktarabilirsiniz.

1GB'dan büyük dışa aktarma dosyaları için [desteğe ulaşın](https://fastcomments.com/auth/my-account/help).

Yeni sitenize FastComments eklemek için [Kurulum rehberine](/guide-installation.html) bakın.