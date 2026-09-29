FastComments'un [WordPress Eklentisi](https://wordpress.org/plugins/fastcomments/) güçlü bir UI tabanlı içe aktarma mekanizmasına sahiptir. Eklentiyi kurduğunuzda, WordPress kurulumunuzu FastComments ile bağlamanıza ve mevcut yorum verilerinizi kopyalamanıza rehberlik edecektir.

**Bu, hiçbir şeyi manuel olarak kopyalamadan veya indirmeden yapılır.**

Göç süreci, göç sırasında UI üzerinden size gösterilecektir. Çoğu göç sadece birkaç dakika sürer.

Mekanizma, göç sırasında WordPress kurulumunuza aşırı yük bindirmeyecek şekilde tasarlanmıştır.

Sitenizi WordPress'ten taşıyorsanız, eklentiyi kullanmak yerine bir WordPress XML veya CSV dışa aktarımını içe aktarabilirsiniz. Bakınız [Yorumlarınızı Yeni Bir Siteye Taşıma](/guide-installation-wordpress.html#wordpress-moving-off-wordpress).

### CloudFlare & Güvenlik Duvarları

Otomatik WordPress kurulumunun çalışabilmesi için WordPress kurulumunuza çağrılar yapmamız gerekir. Cloudflare gibi güvenlik duvarları bizi engelleyebilir ve entegrasyonun başarısız olmasına neden olabilir. Böyle durumlarda, [size bir dizi IP sağlayabiliriz](https://fastcomments.com/auth/my-account/help) entegrasyon için beyaz listeye eklenmek üzere.

### Veri Sahipliği

WordPress göçümüz durumunda, yeni veya güncellenmiş yorum verileri otomatik olarak arka planda WordPress kurulumunuza senkronize edilir. Bu, yorumların FastComments tarafından sunulup WordPress dağıtımınızın yükünü azaltırken, **aynı zamanda** onları bir yedek olarak veritabanınızda sakladığımız anlamına gelir. Bu aynı zamanda FastComments'tan ayrılmak isterseniz, verilerinizin zaten göç edilmiş ve güncel olduğu anlamına gelir.