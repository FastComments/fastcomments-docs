Our [WordPress Plugin](https://wordpress.org/plugins/fastcomments/) güçlü bir UI tabanlı içe aktarma mekanizmasına sahiptir. Eklentiyi kurduğunuzda, WordPress kurulumunuzu FastComments ile bağlamanıza ve mevcut yorum verilerinizi kopyalamanıza rehberlik edecektir.

**Bu, hiçbir şeyi manuel olarak kopyalamadan veya indirmeden yapılır.**

Göç süreci, göç sırasında UI üzerinden size gösterilecektir. Çoğu göç sadece birkaç dakika sürer.

Mekanizma, göç sırasında WordPress kurulumunuza aşırı yük bindirmeyecek şekilde tasarlanmıştır.

Sitenizi WordPress'ten taşıyorsanız, eklentiyi kullanmak yerine bir WordPress XML veya CSV dışa aktarımını içe aktarabilirsiniz. Bakınız [Moving Your Comments to a New Site](/guide-installation-wordpress.html#wordpress-moving-off-wordpress).

### CloudFlare & Güvenlik Duvarları

Otomatik WordPress kurulumunun çalışması için WordPress kurulumunuza çağrılar yapmamız gerekir. Cloudflare gibi güvenlik duvarları bizi engelleyebilir ve entegrasyonun başarısız olmasına neden olabilir. Böyle durumlarda, [size](https://fastcomments.com/auth/my-account/help) entegrasyon için beyaz listeye eklemeniz gereken bir IP seti sağlayabiliriz.

### Veri Sahipliği

WordPress göçümüz durumunda, yeni veya güncellenmiş yorum verileri otomatik olarak arka planda WordPress kurulumunuza senkronize edilir. Bu, yorumların FastComments tarafından sunulup WordPress dağıtımınızın yükünü azaltırken, **aynı zamanda** bir yedek olarak veritabanınızda saklandığı anlamına gelir. Ayrıca, FastComments'tan ayrılmak isterseniz, verilerinizin zaten göç edildiği ve güncel olduğu anlamına gelir.