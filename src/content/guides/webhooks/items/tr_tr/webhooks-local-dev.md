Yerel geliştirme için [ngrok](https://ngrok.com/) gibi bir araç kullanın.

Sistemin güvenliğinin korunmasını kolaylaştırmak için, yerel geliştirme diğer ortamların kurulması ve güvenliğinin sağlanmasıyla aynı süreci izler. 

### Adım 1: Hesabınızdaki alanlara "localhost" ekleyin.

"localhost"u [alan olarak buraya ekleyin](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Adım 2: Bir API Key seçin

Alanınız için webhook yapılandırması ekleyeceğiz, bu yüzden bir API Key'e ihtiyacımız olacak. [Bunu burada yapabilirsiniz.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

"Associate with domain" altında - "localhost" alanınızı seçin.

**NOT: Alternatif olarak, tüm test etkinlikleri ve staging ortamları için tek bir API Secret kullanabilirsiniz. Basitçe "All Domains" için bir API Secret ekleyin ve adını "test" gibi verin.**

Üretim alan(lar)ınız için bir API Secret tanımlı olduğundan emin olun. Diğer tüm alanlar için olaylar wildcard (testing) secret'ını kullanacaktır.

### Adım 3: Webhook'unuzu ekleyin

ngrok veya benzeri bir araç çalışırken, "localhost" için değeri [buraya](https://fastcomments.com/auth/my-account/manage-data/webhooks) ayarlayın.

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

`Send Test Payload`'a tıkladığınızda, API Key'i doğruladığınızı kontrol etmek için iki test olayı göndereceğiz.

Doğrulandıktan sonra `Save`'e basın.

### Adım 4: Bir Yorum Ekleyin

Artık yorum ekleyebilir, düzenleyebilir veya silebilir ve test API Key'inizi kullanarak olaylarla yerel geliştirme makinenizi aradığımızı görebilirsiniz. 
30 saniyeye kadar gecikme olabilir
olayların makinenize ulaşması için.