#### "Kayıt token'ı bulunamadı, süresi doldu veya zaten kullanıldı"

Kayıt URL'nizdeki token (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">buradan alın</a>) 30 dakika geçerlidir ve yalnızca bir kez kullanılabilir. LMS'iniz bundan daha uzun sürdüyse veya kayıt başarılı olduktan sonra tekrar denendiyse, token reddedilecektir. FastComments LTI 1.3 Yapılandırma sayfasında yeni bir URL oluşturun ve baştan başlayın.

#### "Platform kaydı reddetti"

LMS'iniz kayıt el sıkışmasını reddetti. En yaygın nedenler:

- **Araç aynı istemci adıyla zaten kayıtlı.** Bazı platformlar (özellikle D2L), önceki kayıt silinene kadar ikinci bir "FastComments" kaydını reddeder. LMS'inizdeki eski aracı kaldırın ve tekrar deneyin.
- **LMS'te yanlış alan.** URL'yi başlatma URL'si veya oturum açma URL'si alanına değil, **kayıt / araç başlatma kayıt uç noktası** alanına yapıştırdığınızdan emin olun.
- **LMS aslında Dinamik Kayıt'ı desteklemiyor.** Eski Moodle ve Blackboard sürümleri LTI 1.3'ü duyurur ancak yalnızca manuel yapılandırmaya izin verir. Platformunuzun belgelerini kontrol edin.

#### "Platform yapılandırması alınamadı"

FastComments, LMS'inizin openid-configuration belgesini okuyamadı. Bu nadir görülür ve genellikle LMS'in bozuk veya erişilemez bir keşif URL'si sağladığı anlamına gelir. LMS destek ekibinizle iletişime geçin.

#### Başlatma "Configuration not found" gösteriyor

Ya FastComments'teki yapılandırma silindi ya da başlatma, tanımadığımız bir `iss`/`client_id` çiftinden geldi. Silip yeniden kaydettiyseniz, LMS'inize FastComments aracını kaldırmasını ve yeniden eklemesini söyleyin, böylece yeni client_id'yi alır.

#### Başlatma "Deployment not registered" gösteriyor

FastComments'i, ilk başlatıldığı dağıtımdan farklı bir Brightspace/Moodle/Blackboard dağıtımından başlattınız. FastComments, güvenlik kontrolü olarak `deployment_id`'yi ilk başlatmada sabitler. Aynı istemci altına yeni bir dağıtım eklemek için destek ekibiyle iletişime geçin - dağıtım ID'sini yapılandırmaya ekleyeceğiz.

#### Başlatma "Unsupported message_type" gösteriyor

LMS, FastComments'in işlemediği bir LTI mesajı gönderdi (örn. `LtiSubmissionReviewRequest`). FastComments yalnızca standart kaynak bağlantısı başlatma ve derin bağlantı akışlarını destekler. Belirli bir mesaj türünün eklenmesini gerektiriyorsanız bize ulaşın.

#### Iframe yeniden boyutlandırılmıyor

Çoğu LMS, LTI iframe'lerini otomatik boyutlandırır. Sizin LMS'iniz yapmıyorsa, LMS'in başlatma ayarlarının aracın üst çerçeveye postMessage olayları göndermesine izin verip vermediğini kontrol edin. FastComments hem Canvas tarzı (`lti.frameResize`) hem de IMS-spec (`org.imsglobal.lti.frameResize`) yeniden boyutlandırma mesajları gönderir.