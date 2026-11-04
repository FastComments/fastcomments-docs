[WordPress eklentimiz](https://wordpress.org/plugins/fastcomments/), güçlü bir arayüz tabanlı içe aktarma mekanizmasına sahiptir. Eklentiyi yükledikten sonra,
WordPress kurulumunuzu FastComments ile bağlama ve mevcut yorum verilerinizi kopyalama adımlarında size rehberlik edecektir.

**Bu işlem, hiçbir şeyi manuel olarak kopyalamayı veya indirmeyi gerektirmeden yapılır.**

Geçiş süreci, geçiş sırasında arayüz üzerinden size gösterilecektir. Geçişlerin çoğu yalnızca birkaç dakika sürer.

Bu mekanizma, geçiş sırasında WordPress kurulumunuza aşırı yük bindirmemek üzere tasarlanmıştır.

Sitenizi WordPress'ten taşıyorsanız, eklentiyi kullanmak yerine bir WordPress XML veya CSV dışa aktarımını içe aktarabilirsiniz. Bkz.
[Yorumlarınızı Yeni Bir Siteye Taşımak](/guide-installation-wordpress.html#wordpress-moving-off-wordpress).

### CloudFlare ve Güvenlik Duvarları

Otomatik WordPress kurulumunun çalışabilmesi için WordPress kurulumunuza çağrılar yapmamız gerekir.
Cloudflare gibi güvenlik duvarları bizi engelleyebilir ve entegrasyonun başarısız olmasına neden olabilir. Bu gibi durumlarda, entegrasyon için
beyaz listeye eklemeniz gereken bir IP seti [size sağlayabiliriz](https://fastcomments.com/auth/my-account/help).

### Veri Sahipliği

WordPress geçişimiz söz konusu olduğunda, yeni veya güncellenen yorum verileri arka planda otomatik olarak WordPress kurulumunuza geri senkronize edilir.
Bu, yorumlar WordPress dağıtımınızın yükünü hafifletmek için FastComments tarafından sunulsa da, bunları **aynı zamanda** yedek olarak veritabanınıza da kaydettiğimiz anlamına gelir.
Bu aynı zamanda, FastComments'ten ayrılmak isterseniz verilerinizin zaten taşınmış ve güncel olduğu anlamına gelir.