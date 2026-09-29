FastComments'un [WordPress Eklentisi](https://wordpress.org/plugins/fastcomments/) güçlü bir UI tabanlı içe aktarma mekanizmasına sahiptir. Eklentiyi kurduğunuzda, WordPress kurulumunuzu FastComments ile bağlamanıza ve mevcut yorum verilerinizi kopyalamanıza rehberlik edecektir.

**Bu, hiçbir şeyi manuel olarak kopyalamadan veya indirmeden yapılır.**

Göç süreci, göç sırasında UI üzerinden size gösterilecektir. Çoğu göç sadece birkaç dakika sürer.

Mekanizma, göç sırasında WordPress kurulumunuza aşırı yük bindirmeyecek şekilde tasarlanmıştır.

Sitenizi WordPress'ten taşıyorsanız, eklentiyi kullanmak yerine bir WordPress XML veya CSV dışa aktarımını içe aktarabilirsiniz. Bkz. [Yorumlarınızı Yeni Bir Siteye Taşıma](/guide-installation-wordpress.html#wordpress-moving-off-wordpress).

### CloudFlare & FireWalls

Otomatik WordPress kurulumunun çalışabilmesi için WordPress kurulumunuza isteklerde bulunmamız gerekir. Cloudflare gibi güvenlik duvarları bizi engelleyebilir ve entegrasyonun başarısız olmasına neden olabilir. Bu gibi durumlarda, entegrasyon için beyaz listeye eklemeniz gereken IP setini [size sağlayabiliriz](https://fastcomments.com/auth/my-account/help).

### Data Ownership

WordPress göçümüz durumunda, yeni veya güncellenmiş yorum verileri otomatik olarak arka planda WordPress kurulumunuza senkronize edilir. Bu, yorumların FastComments tarafından sunularak WordPress dağıtımınızın yükünü azaltırken, **aynı zamanda** bunları bir yedek olarak veritabanınızda sakladığımız anlamına gelir. Ayrıca, FastComments'tan ayrılmak isterseniz, verilerinizin zaten göç edildiği ve güncel olduğu anlamına gelir.