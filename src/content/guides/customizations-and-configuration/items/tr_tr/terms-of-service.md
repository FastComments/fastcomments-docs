FastComments, ilk kez yorum yapanların bir yorumu göndermeden önce Hizmet Şartlarınızı kabul etmelerini zorunlu kılmanıza olanak tanır.

Etkinleştirildiğinde:
- **Anonim kullanıcılar** her yorum yaptıklarında bir TOS onay kutusu görecekler
- **Kimlik doğrulanmış kullanıcılar** onay kutusunu yalnızca ilk yorumlarında veya TOS'u güncellediğinizde görecekler

### Yapılandırma

Widget özelleştirme sayfasına gidin ve "Hizmet Şartları kabulünü zorunlu kıl" onay kutusunu etkinleştirin. Etkinleştirildikten sonra aşağıdaki seçenekleri göreceksiniz:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.tos-enabled'; selector = '#tos-options'; title='Terms of Service Options' app-screenshot-end]

- **TOS Metin Modu**: Varsayılan olarak, onay kutusu her iki belgeye bağlantılar içeren "Hizmet Şartları ve Gizlilik Politikasını kabul ediyorum" ifadesini gösterir. Her dil için kendi metninizi sağlamak üzere "Her dil için metni özelleştir" seçeneğini seçin.
- **TOS Son Güncelleme Tarihi**: Hizmet Şartlarınızı güncellediğinizde bu tarihi ayarlayın. Bu tarihten önce kabul eden kullanıcıların tekrar kabul etmeleri gerekecektir.

### Nasıl Çalışır

- TOS kabul zaman damgası kullanıcı başına ve yorum başına kaydedilir
- Bir kullanıcı TOS'u kabul ettiğinde, tarih kullanıcı profiline (kiracı başına) kaydedilir
- Kullanıcının kabul tarihinden sonra bir "Son Güncelleme" tarihi ayarlarsanız, onların yeniden kabul etmeleri gerekir
- İzlenemeyen anonim kullanıcılar için, onay kutusu her yorum gönderiminde görünür