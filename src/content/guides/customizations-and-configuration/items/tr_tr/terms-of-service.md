FastComments, ilk kez yorum yapan kullanıcıların yorum göndermeden önce Hizmet Şartlarınızı kabul etmesini zorunlu kılmanıza olanak tanır.

Etkinleştirildiğinde:
- **Anonim kullanıcılar** her yorum yaptıklarında bir TOS onay kutusu görecek
- **Kimliği doğrulanmış kullanıcılar** onay kutusunu yalnızca ilk yorumlarında veya TOS'unuzu güncellediğinizde görecek

### Yapılandırma

Widget özelleştirme sayfasına gidin ve "Hizmet Şartları kabulünü zorunlu kıl" onay kutusunu etkinleştirin. Etkinleştirildiğinde aşağıdaki seçenekleri göreceksiniz:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.tos-enabled'; selector = '#tos-options'; alt='TOS metin modu seçicisini ve son güncellenme tarihi alanını gösteren Hizmet Şartları paneli'; title='Hizmet Şartları Seçenekleri' app-screenshot-end]

- **TOS Metin Modu**: Varsayılan olarak, onay kutusu her iki belgeye bağlantı içeren "Hizmet Şartları ve Gizlilik Politikası'nı kabul ediyorum" metnini görüntüler. Her dil için kendi metninizi sağlamak üzere "Yerel ayara göre metni özelleştir" seçeneğini seçin.
- **TOS Son Güncellenme Tarihi**: Hizmet Şartlarınızı güncellediğinizde bu tarihi ayarlayın. Bu tarihten önce kabul etmiş kullanıcıların yeniden kabul etmesi gerekecek.

### Nasıl Çalışır

- TOS kabul zaman damgası kullanıcı başına ve yorum başına saklanır
- Bir kullanıcı TOS'u kabul ettiğinde, tarih kullanıcı profilinde (kiracı başına) kaydedilir
- Kullanıcının kabul tarihinden sonra bir "Son Güncellenme" tarihi ayarlarsanız, yeniden kabul etmeleri gerekecek
- Takip edilemeyen anonim kullanıcılar için, onay kutusu her yorum gönderiminde görünür