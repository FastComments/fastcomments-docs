FastComments hem SSO hem de SAML kimlik doğrulamasını sunar. Farkları anlamak, kuruluşunuz için doğru yaklaşımı seçmenize yardımcı olur.

### Basit/Güvenli SSO Çözümleri

FastComments, siteniz üzerinden yorum bileşenine kimlik doğrulaması yapmak için iki farklı SSO akışı sunar.
Bu, SAML'den farklıdır ve SAML gerektirmez. Bunun yerine, Basit SSO yalnızca yorum bileşenine bir nesne geçirmeyi gerektirirken,
Güvenli SSO bunu yapar ve ayrıca yükü bir API anahtarı ile hash'ler.

Öte yandan SAML, kullanıcıyı ürünün tamamına (izinlerine bağlı olarak) *ve* yorum bileşenine doğrular
(eğer etki alanımız için üçüncü taraf çerezlerini etkinleştirmişlerse).

### SAML Kimlik Doğrulama

SAML, daha sağlam güvenlik ve entegrasyon yetenekleri sağlayan kurumsal düzeyde bir kimlik doğrulama protokolüdür:

- **Uygulama**: Kimlik Sağlayıcı (IdP) yapılandırması ve sertifika değişimini gerektirir
- **Güvenlik**: İmzalı XML doğrulamalarını kullanır ve şifrelemeyi destekler
- **Kullanım Durumu**: Mevcut SAML altyapısına (Active Directory, Okta vb.) sahip işletmeler için idealdir
- **Kurulum Karmaşıklığı**: Daha kapsamlıdır - IdP yapılandırması ve sertifika yönetimi gerektirir
- **Kurumsal Özellikler**: Gelişmiş rol eşleştirme, merkezi kullanıcı yönetimi, denetim izleri

### Ne Zaman SAML Seçilmeli

Kuruluşunuz aşağıdakilere uyuyorsa SAML kimlik doğrulamasını değerlendirin:

- Zaten SAML uyumlu bir kimlik sağlayıcı (Okta, Azure AD, ADFS, vb.) kullanıyorsa
- Kurumsal düzeyde güvenlik ve uyumluluk gerekiyorsa
- Merkezi kullanıcı yönetimi ve erişim kontrolüne ihtiyaç duyuyorsa
- Kimlik doğrulama için SAML kullanan birden fazla uygulamaya sahipse
- Ayrıntılı denetim izleri ve güvenlik raporlaması gerekiyorsa

### Basit veya Güvenli SSO Ne Zaman Seçilmeli

Widget odaklı SSO çözümlerimiz yeterli olabilir eğer siz:

- Özel bir kimlik doğrulama sistemine sahipseniz
- Minimum kurulumla hızlı bir uygulamaya ihtiyaç duyuyorsanız
- Kurumsal kimlik sağlayıcı entegrasyonuna ihtiyacınız yoksa
- Kullanıcı verilerini doğrudan uygulamanızdan kontrol etmek istiyorsanız
- Daha basit güvenlik gereksinimleriniz varsa

Basit ve Güvenli SSO, kullanıcıların zaten hesabı olduğu çevrimiçi portallar, bloglar vb. için yaygın olarak kullanılır (*siteleriniz veya uygulamanız aracılığıyla*)
ancak mutlaka SAML kullanmayabilirler.