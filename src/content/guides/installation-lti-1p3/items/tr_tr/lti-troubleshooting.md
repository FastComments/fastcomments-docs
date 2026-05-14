#### "Kayıt tokeni bulunamadı, süresi doldu veya zaten kullanılmış"

Kayıt URL'nizdeki token 30 dakika için geçerlidir ve yalnızca bir kez kullanılabilir. LMS'iniz bunun için daha uzun sürdüyse veya kayıt başarıyla tamamlandıktan sonra tekrar denendiyse, token reddedilecektir. FastComments LTI 1.3 Yapılandırma sayfasında yeni bir URL oluşturun ve yeniden başlayın.

#### "Platform kaydı reddetti"

LMS'iniz kayıt el sıkışmasını reddetti. En yaygın nedenler:

- **Aynı istemci adıyla araç zaten kayıtlı.** Bazı platformlar (özellikle D2L), önceki araç silinene kadar "FastComments"ın ikinci bir kaydını reddeder. LMS'inizde eski aracı kaldırın, sonra tekrar deneyin.
- **LMS'deki yanlış alan.** URL'yi launch URL veya login URL alanına değil, **registration / tool initiation registration endpoint** alanına yapıştırdığınızdan emin olun.
- **LMS aslında Dinamik Kaydı desteklemiyor.** Eski Moodle ve Blackboard sürümleri LTI 1.3'ü duyurur ama yalnızca manuel yapılandırmaya izin verir. Platformunuzun belgelerini kontrol edin.

#### "Platform yapılandırması alınamadı"

FastComments LMS'inizin openid-configuration belgesini okuyamadı. Bu nadirdir ve genellikle LMS'in hatalı biçimlendirilmiş veya ulaşılamayan bir discovery URL'si sağladığı anlamına gelir. LMS desteğinizle iletişime geçin.

#### Başlatma "Yapılandırma bulunamadı" gösteriyor

Ya FastComments içindeki yapılandırma silindi ya da başlatma, bizim tanımadığımız bir `iss`/`client_id` çifti tarafından yapıldı. Eğer yapılandırmayı silip tekrar kaydettiyseniz, LMS'inize FastComments aracını kaldırıp yeniden eklemesini söyleyin, böylece yeni client_id'yi alır.

#### Başlatma "Deployment not registered" gösteriyor

FastComments'ı, ilk başlatıldığı dağıtımdan farklı bir Brightspace/Moodle/Blackboard dağıtımından başlattınız. FastComments, ilk başlatmada güvenlik kontrolü olarak `deployment_id`'yi sabitler. Aynı istemci altında yeni bir dağıtım eklemek için destek ile iletişime geçin - yapılandırmaya deployment ID'sini biz ekleriz.

#### Başlatma "Unsupported message_type" gösteriyor

LMS, FastComments'ın işlemediği bir LTI mesajı gönderdi (örn. `LtiSubmissionReviewRequest`). FastComments yalnızca standart resource-link başlatma ve deep-linking akışlarını destekler. Belirli bir mesaj türünün eklenmesine ihtiyacınız varsa bizimle iletişime geçin.

#### Iframe yeniden boyutlandırılmıyor

Çoğu LMS, LTI iframe'lerini otomatik boyutlandırır. Sizinki yapmıyorsa, LMS'in başlatma ayarlarının aracın parent frame'e postMessage olayları göndermesine izin verdiğini kontrol edin. FastComments hem Canvas tarzı (`lti.frameResize`) hem de IMS-spec (`org.imsglobal.lti.frameResize`) yeniden boyutlandırma mesajları gönderir.