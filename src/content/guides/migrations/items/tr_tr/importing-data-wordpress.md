WordPress Eklentimiz (https://wordpress.org/plugins/fastcomments/) güçlü bir kullanıcı arayüzü tabanlı içe aktarma mekanizmasına sahiptir. Eklentiyi kurduğunuzda,
WordPress kurulumunuzu FastComments ile bağlamanıza ve mevcut yorum verilerinizi kopyalamanıza rehberlik edecektir.

**Bu işlem herhangi bir şeyi manuel olarak kopyalamadan veya indirmeden yapılır.**

Taşıma işlemi, taşıma sırasında kullanıcı arayüzü aracılığıyla size bildirilecektir. Çoğu taşıma sadece birkaç dakika sürer.

Bu mekanizma, taşıma sırasında WordPress kurulumunuza aşırı yük bindirmeyecek şekilde tasarlanmıştır.

### CloudFlare & Güvenlik Duvarları

Otomatik WordPress kurulumunun çalışabilmesi için WordPress kurulumunuza çağrılar yapmamız gerekiyor.
Cloudflare gibi güvenlik duvarları bizi engelleyebilir ve entegrasyonun başarısız olmasına neden olabilir. Bu gibi durumlarda, [size
sağlayabiliriz](https://fastcomments.com/auth/my-account/help) entegrasyon için beyaz listeye eklemeniz gereken bir IP listesi ile.

### Veri Sahipliği

WordPress taşıma işlemimizde, yeni veya güncellenmiş tüm yorum verileri arka planda otomatik olarak WordPress kurulumunuza senkronize edilir.
Bu, yorumlar WordPress dağıtımınıza binen yükü azaltmak için FastComments tarafından servis edilirken, bunları veritabanınızda yedek olarak **ayrıca** sakladığımız anlamına gelir. Bu ayrıca FastComments'tan ayrılmak isterseniz verilerinizin zaten taşınmış ve güncel olduğu anlamına gelir.