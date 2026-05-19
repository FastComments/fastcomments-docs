#### "Kayıt belirteci bulunamadı, süresi doldu veya zaten kullanıldı"

Kayıt URL'nizdeki belirteç (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">buradan alabilirsiniz</a>) 30 dakika için geçerlidir ve yalnızca bir kez kullanılabilir. LMS'iniz bunun üzerinde sürdüyse veya kayıt başarıyla gerçekleştirildikten sonra yeniden denendiyse, belirteç reddedilir. FastComments LTI 1.3 Yapılandırma sayfasından yeni bir URL oluşturun ve yeniden başlayın.

#### "Platform kaydı reddetti"

LMS'iniz kayıt el sıkışmasını reddetti. En yaygın nedenler:

- **Aynı istemci adıyla araç zaten kayıtlı.** Bazı platformlar (özellikle D2L), önceki silinene kadar "FastComments" için yapılan ikinci kaydı reddeder. LMS'inizdeki eski aracı kaldırın, ardından yeniden deneyin.
- **LMS'de yanlış alan.** URL'yi launch URL veya login URL alanına değil, **registration / tool initiation registration endpoint** alanına yapıştırdığınızdan emin olun.
- **LMS gerçekte Dynamic Registration'ı desteklemiyor.** Eski Moodle ve Blackboard sürümleri LTI 1.3'ü ilan eder ancak yalnızca manuel yapılandırmaya izin verir. Platformunuzun belgelerini kontrol edin.

#### "Platform yapılandırması alınamadı"

FastComments, LMS'inizin openid-configuration belgesini okuyamadı. Bu nadirdir ve genellikle LMS'in hatalı biçimlendirilmiş veya ulaşılamayan bir discovery URL'si sağladığı anlamına gelir. LMS destek ekibinizle iletişime geçin.

#### Launch shows "Configuration not found"

Ya FastComments içindeki yapılandırma silindi, ya da başlatma bizim tanımadığımız bir `iss`/`client_id` çiftinden geldi. Eğer sildiyseniz ve yeniden kaydettiyseniz, LMS'inize eski FastComments aracını kaldırıp yeniden eklemesini söyleyin, böylece yeni client_id alınır.

#### Launch shows "Deployment not registered"

FastComments'i ilk başlatıldığı dağıtımdan farklı bir Brightspace/Moodle/Blackboard dağıtımından başlattınız. FastComments ilk başlatmada bir güvenlik kontrolü olarak `deployment_id`'yi sabitler. Aynı client altında yeni bir dağıtım eklemek için destek ile iletişime geçin - yapılandırmaya deployment ID'sini ekleyeceğiz.

#### Launch shows "Unsupported message_type"

LMS, FastComments'in işleyemediği bir LTI mesajı gönderdi (ör. `LtiSubmissionReviewRequest`). FastComments yalnızca standart resource-link başlatmasını ve deep-linking akışlarını destekler. Belirli bir mesaj türünün eklenmesini istiyorsanız bize ulaşın.

#### Iframe doesn't resize

Çoğu LMS, LTI iframe'lerini otomatik boyutlandırır. Eğer sizinki yapmıyorsa, LMS'in başlatma ayarlarının aracın üst çerçeveye postMessage olayları göndermesine izin verdiğini kontrol edin. FastComments hem Canvas tarzı (`lti.frameResize`) hem de IMS belirtimi (`org.imsglobal.lti.frameResize`) yeniden boyutlandırma mesajları gönderir.