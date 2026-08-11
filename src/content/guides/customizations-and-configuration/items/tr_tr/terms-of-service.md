FastComments, ilk kez yorum yapanların bir yorum göndermeden önce Hizmet Şartlarınızı kabul etmelerini zorunlu kılmanıza olanak tanır.

Etkinleştirildiğinde:
- **Anonim kullanıcılar** her yorum yaptıklarında bir TOS onay kutusu göreceklerdir
- **Kimliği doğrulanmış kullanıcılar** yalnızca ilk yorumlarında veya Hizmet Şartlarınızı güncellediğinizde onay kutusunu göreceklerdir

### Configuration

Widget özelleştirme sayfasına gidin ve "Hizmet Şartları kabulünü zorunlu kıl" onay kutusunu etkinleştirin. Etkinleştirildiğinde, aşağıdaki seçenekleri göreceksiniz:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.tos-enabled'; selector = '#tos-options'; alt='Hizmet Şartları paneli, TOS metin modu seçicisini ve son güncellenme tarih alanını gösteriyor'; title='Hizmet Şartları Seçenekleri' app-screenshot-end]

- **TOS Metin Modu**: Varsayılan olarak, onay kutusu "Hizmet Şartları ve Gizlilik Politikasını kabul ediyorum" metnini her iki belgeye bağlantılarla gösterir. Her dil için kendi metninizi sağlamak üzere "Yerel bazında metni özelleştir" seçeneğini seçin.
- **TOS Son Güncelleme Tarihi**: Hizmet Şartlarınızı güncellediğinizde bu tarihi ayarlayın. Bu tarihten önce kabul eden kullanıcıların tekrar kabul etmeleri gerekecektir.

### How It Works

- TOS kabul zaman damgası kullanıcı başına ve yorum başına depolanır
- Bir kullanıcı TOS'u kabul ettiğinde, tarih kullanıcı profiline (per-tenant) kaydedilir
- Kullanıcının kabul tarihinden sonra bir "Last Updated" tarihi belirlerseniz, yeniden kabul etmeleri gerekir
- İzlenemeyen anonim kullanıcılar için, onay kutusu her yorum gönderiminde görünür