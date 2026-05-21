### Shopify App Store'dan yükleyin

1. [Shopify App Store'daki FastComments sayfasını](https://apps.shopify.com/fastcomments) açın.
2. **Uygulamayı ekle**'ye tıklayın ve kurulum akışı sırasında istediğiniz planı seçin.
3. Kurulum tamamlandığında Shopify sizi Shopify içindeki FastComments yönetim paneline geri yönlendirir.

Kurulum bu kadar. Tema dosyalarınıza yapıştırmanız gereken bir şey yok.

### Sizin için nelerin kuruldu

Yükleme, aksi takdirde elle yapacağınız tüm adımları yürütür:

- Mağazanız için bir FastComments tenant oluşturulur ve mağaza alan adınıza bağlanır.
- Mağazanızın URL'si tenant'ın yetkilendirilmiş alan adlarına eklenir, böylece yorumlar alan adı hatası olmadan yüklenir.
- Her bloğun hangi tenant'a göre görüntüleneceğini bilmesi için `fastcomments.tenant_id` shop metafield yazılır.
- Shopify müşterileriniz için tek oturum açma (single sign-on) varsayılan olarak etkinleştirilir.
- Faturalandırma Shopify Managed Pricing üzerinden yürütülür. Ücretler standart Shopify faturanıza yansır. Shopify yönetici panelinizde **Ayarlar > Uygulamalar ve satış kanalları > FastComments** bölümünden yükseltme, düşürme veya iptal yapabilirsiniz.

Eğer mağazanız uygulamayı yüklemeden önce zaten FastComments müşterisiyse, kurulum yeni bir tenant oluşturmaktansa mevcut tenant'ı tekrar kullanır.

### Gömülü yönetim paneli

Shopify yönetici panelinizden FastComments uygulamasını açtığınızda, sizi tam FastComments arka ucuna tek tıkla erişim sağlayan bir pano karşılar:

- **Dashboard**: hesap ayarları, kullanım ve abonelik detayları.
- **Moderasyon Kuyruğu**: mağazanız genelindeki yorumları onaylayın, reddedin ve yanıtlayın.
- **Özelleştir**: widget renklerini, yazı tiplerini, moderasyon kurallarını ve yapılandırmayı ayarlayın.
- **Puanlar & İncelemeler Yardımcısı**: yıldız puanlarını ve inceleme sorularını ayarlayın; Reviews Summary bloğunu kullanmak isterseniz.

Her kutucuk FastComments'i tek kullanımlık bir oturum açma bağlantısıyla açar, böylece ayrı bir oturum açmanıza gerek kalmaz.

### Sonraki: mağazanıza bloklar ekleyin

Shopify tema düzenleyicinizi açın (**Çevrimiçi Mağaza > Temalar > Özelleştir**), yorum veya inceleme eklemek istediğiniz şablonu açın ve **Blok ekle**'ye tıklayın. FastComments blokları **Uygulamalar** altında görünür. Bu kılavuzun geri kalanı her bir bloğu kapsar.