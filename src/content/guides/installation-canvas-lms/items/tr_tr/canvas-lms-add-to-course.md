#### Yorumların Kurslarınızda Nasıl Göründüğü

LTI entegrasyonu etkinleştirildiğinde ve Harici Uygulama yüklendiğinde, FastComments yapılandırdığınız yerleştirmelere göre otomatik olarak çalışır:

#### Ödev Görünümü

Eğer **Assignment View** yerleştirmesi etkinse, yorumlar kurs içindeki her ödevin altında otomatik olarak görünür. Öğrenciler ve eğitmenler bir ödevi görüntülediklerinde bir konu başlıklı yorum bölümü görürler — ödev başına ekstra bir kurulum gerekmez.

Her ödev kendi ayrı yorum konusuna sahiptir.

#### Zengin İçerik Düzenleyici Düğmesi

Eğer **Editor Button** yerleştirmesi etkinse, eğitmenler FastComments'i Zengin İçerik Düzenleyicisi kullanan herhangi bir içeriğe gömebilirler:

1. **Sayfa**, **Quiz** veya **Duyuru** düzenleyin.  
2. Zengin İçerik Düzenleyicisi araç çubuğunda, **FastComments** düğmesine tıklayın.  
3. FastComments otomatik olarak içeriğe gömülür.  
4. Sayfayı kaydedin.

Öğrenciler sayfayı görüntülediğinde, gömülü FastComments widget'ı o sayfaya özgü bir yorum konusuyla yüklenir.

#### Otomatik SSO

Her iki yerleştirmede de, öğrenciler Canvas hesaplarıyla otomatik olarak oturum açar. İsimler, e-posta adresleri ve avatarlar LTI başlatmasıyla senkronize edilir, ayrı bir giriş gerekmez.

#### Genel Erişimi Kapatma (Önerilir)

Varsayılan olarak, FastComments yorum verileri herkese açık olarak okunabilir. Bir konunun URL'sini veya API uç noktasını tahmin edebilen herkes, Canvas dışındaki durumlarda bile yorumları görebilir. Kurs tartışmaları için muhtemelen görüntülemeyi yalnızca kayıtlı öğrencilere sınırlamak istersiniz.

<a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">widget özelleştirme sayfası</a>nı açın ve **Require SSO To View Comments** etkinleştirilmiş bir kural oluşturun, ardından güvenlik seviyesini **Secure SSO** olarak ayarlayın, böylece konular yalnızca imzalı LTI başlatmasıyla yüklenebilir.

Tam kılavuz için [Tek Oturum Açma ile Yorum Konularını Koruma](/guide-customizations-and-configuration.html#sso-require-to-view-comments) sayfasına bakın, kuralı tek bir alan adı veya sayfaya nasıl uygulayacağınızı da içerir.