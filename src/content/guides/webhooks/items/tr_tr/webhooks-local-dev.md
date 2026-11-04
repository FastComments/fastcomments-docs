Yerel geliştirme için [ngrok](https://ngrok.com/) gibi bir araç kullanın.

Sistemi güvenli tutmayı basitleştirmek için, yerel geliştirme diğer ortamları kurma ve güvenli hale getirme sürecinin aynısını takip eder.

### Adım 1: Hesabınızdaki alan adlarına "localhost" ekleyin.

"localhost" alan adını [buradan ekleyin](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='Alan adı alanına localhost girilmiş hesap ayarlarındaki alan adı ekleme formu'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Adım 2: Bir API Anahtarı Seçin

Alan adınız için webhook yapılandırması ekleyeceğiz, bu yüzden bir API anahtarına ihtiyacımız olacak. [Bunu buradan yapabilirsiniz.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='İlişkili alan adı localhost olarak ayarlanmış ve anahtara Testing adı verilmiş yeni API gizli anahtarı formu'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

"Associate with domain" altında - "localhost" alan adınızı seçin.

**NOT: Alternatif olarak, tüm test etkinlikleri ve hazırlık (staging) ortamları için tek bir API Gizli Anahtarı kullanabilirsiniz. Sadece "All Domains" için bir API Gizli Anahtarı ekleyin ve ona "test" gibi bir ad verin.**

Üretim alan ad(lar)ınız için tanımlanmış bir API Gizli Anahtarı olduğundan emin olun. Diğer tüm alan adlarının etkinlikleri joker (test) gizli anahtarını kullanacaktır.

### Adım 3: Webhook'unuzu Ekleyin

ngrok veya benzeri bir araç çalışırken, "localhost" için değeri [buradan](https://fastcomments.com/auth/my-account/manage-data/webhooks) ayarlayın.

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; alt='localhost alan adının seçilmiş olduğu ve comment created uç noktasına bir ngrok URL\'si girilmiş webhooks yönetim ekranı'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

`Send Test Payload` tıklandığında, API anahtarını doğruladığınızı kontrol etmek için size iki test etkinliği göndereceğiz.

Doğrulama tamamlandıktan sonra `Save` tuşuna basın.

### Adım 4: Bir Yorum Ekleyin

Artık yorum ekleyebilir, düzenleyebilir veya silebilirsiniz ve etkinliklerle birlikte test API anahtarınızı kullanarak yerel geliştirme makinenizi aradığımızı görmelisiniz. Etkinliklerin makinenize ulaşması 30 saniye kadar sürebilir.