FastComments'un [WordPress Eklentisi](https://wordpress.org/plugins/fastcomments/) güçlü bir UI tabanlı içe aktarma mekanizmasına sahiptir. Eklentiyi kurduğunuzda,  
size WordPress kurulumunuzu FastComments ile bağlamanız ve mevcut yorum verilerinizi kopyalamanız konusunda rehberlik edecektir.

**Bu, hiçbir şeyi manuel olarak kopyalamadan veya indirmeden yapılır.**

Geçiş süreci, UI üzerinden size gösterilecektir. Çoğu geçiş sadece birkaç dakika sürer.

Bu mekanizma, geçiş sırasında WordPress kurulumunuza aşırı yük bindirmeyecek şekilde tasarlanmıştır.

Sitenizi WordPress'ten taşıyorsanız, eklentiyi kullanmak yerine bir WordPress XML veya CSV dışa aktarımını içe aktarabilirsiniz. Bakınız [Yorumlarınızı Yeni Bir Siteye Taşıma](/guide-installation-wordpress.html#wordpress-moving-off-wordpress).

### CloudFlare & Güvenlik Duvarları

Otomatik WordPress kurulumunun çalışması için WordPress kurulumunuza çağrılar yapmamız gerekir. Cloudflare gibi güvenlik duvarları bizi engelleyebilir ve entegrasyonun başarısız olmasına neden olabilir. Böyle durumlarda, [size bir dizi IP sağlayabiliriz](https://fastcomments.com/auth/my-account/help) entegrasyon için beyaz listeye eklemeniz amacıyla.

### Veri Sahipliği

WordPress geçişimiz durumunda, yeni veya güncellenmiş yorum verileri otomatik olarak arka planda WordPress kurulumunuza senkronize edilir. Bu, yorumların FastComments tarafından sunulup WordPress dağıtımınızın yükünü azaltırken, **aynı zamanda** onları bir yedek olarak veritabanınızda sakladığımız anlamına gelir. Bu aynı zamanda FastComments'tan ayrılmak isterseniz, verilerinizin zaten geçiş yapmış ve güncel olduğu anlamına gelir.