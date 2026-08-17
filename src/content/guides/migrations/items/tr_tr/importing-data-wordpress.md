Our [WordPress Eklentisi](https://wordpress.org/plugins/fastcomments/) güçlü bir UI tabanlı içe aktarma mekanizmasına sahiptir. Eklentiyi kurduğunuzda,
WordPress kurulumunuzu FastComments ile bağlamanıza ve mevcut yorum verilerinizi kopyalamanıza rehberlik edecektir.

**Bu, hiçbir şeyi manuel olarak kopyalamadan veya indirmeden yapılır.**

Göç süreci, UI üzerinden size gösterilecektir. Çoğu göç sadece birkaç dakika sürer.

Mekanizma, göç sırasında WordPress kurulumunuza aşırı yük bindirmeyecek şekilde tasarlanmıştır.

### CloudFlare & Güvenlik Duvarları

Otomatik WordPress kurulumunun çalışması için WordPress kurulumunuza çağrılar yapmamız gerekir.
Cloudflare gibi güvenlik duvarları bizi engelleyebilir ve entegrasyonun başarısız olmasına neden olabilir. Böyle durumlarda, [size](https://fastcomments.com/auth/my-account/help) entegrasyon için beyaz listeye eklenmesi gereken IP setini sağlayabiliriz.

### Veri Sahipliği

WordPress göçümüz durumunda, yeni veya güncellenen yorum verileri otomatik olarak arka planda WordPress kurulumunuza senkronize edilir.
Bu, yorumların FastComments tarafından sunulup WordPress dağıtımınızın yükünü hafifletirken,
bizim **aynı zamanda** bir yedek olarak veritabanınıza kaydettiğimiz anlamına gelir. Bu aynı zamanda FastComments'tan ayrılmak isterseniz, verilerinizin zaten göç edilmiş ve güncel olduğu anlamına gelir.