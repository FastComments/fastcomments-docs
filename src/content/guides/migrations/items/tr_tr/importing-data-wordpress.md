FastComments'un [WordPress Eklentisi](https://wordpress.org/plugins/fastcomments/) güçlü bir UI tabanlı içe aktarma mekanizmasına sahiptir. Eklentiyi kurduğunuzda,
WordPress kurulumunuzu FastComments ile bağlamanıza ve mevcut yorum verilerinizi kopyalamanıza rehberlik eder.

**Bu, hiçbir şeyi manuel olarak kopyalamadan veya indirmeden yapılır.**

Taşıma süreci, UI üzerinden size gösterilecektir. Çoğu taşıma sadece birkaç dakika sürer.

Mekanizma, taşıma sırasında WordPress kurulumunuza aşırı yük bindirmeyecek şekilde tasarlanmıştır.

Sitenizi WordPress'ten taşıyorsanız, eklentiyi kullanmak yerine bir WordPress XML veya CSV dışa aktarımını içe aktarabilirsiniz. Bakınız
[Yorumlarınızı Yeni Bir Siteye Taşıma](/guide-installation-wordpress.html#wordpress-moving-off-wordpress).

### CloudFlare & FireWalls

Otomatik WordPress kurulumunun çalışabilmesi için WordPress kurulumunuza çağrılar yapmamız gerekir.
Cloudflare gibi güvenlik duvarları bizi engelleyebilir ve entegrasyonun başarısız olmasına neden olabilir. Bu gibi durumlarda, [size](https://fastcomments.com/auth/my-account/help) entegrasyon için beyaz listeye eklenmesi gereken IP setini sağlayabiliriz.

### Data Ownership

WordPress taşıma işlemimizde, yeni veya güncellenmiş yorum verileri otomatik olarak arka planda WordPress kurulumunuza senkronize edilir.
Bu, yorumların FastComments tarafından sunularak WordPress dağıtımınızın yükünü hafifletirken,
biz **aynı zamanda** bunları bir yedek olarak veritabanınızda saklarız anlamına gelir. Ayrıca, FastComments'tan ayrılmak isterseniz, verilerinizin zaten taşınmış ve güncel olduğu anlamına gelir.