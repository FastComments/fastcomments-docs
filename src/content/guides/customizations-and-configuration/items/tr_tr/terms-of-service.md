FastComments, ilk kez yorum yapanların yorum göndermeden önce Hizmet Şartlarınızı kabul etmelerini zorunlu kılmanıza olanak tanır.

When enabled:
- **Anonim kullanıcılar** her yorum yaptıklarında bir Hizmet Şartları onay kutusu görecekler
- **Kimliği doğrulanmış kullanıcılar** onay kutusunu yalnızca ilk yorumlarında veya Hizmet Şartlarınızı güncellediğinizde görecekler

### Hizmet Şartlarını Etkinleştirme

Bileşen özelleştirme sayfasına gidin ve "Hizmet Şartlarının kabulünü zorunlu kıl" onay kutusunu etkinleştirin:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-enabled'; title='Enable Terms of Service Checkbox' app-screenshot-end]

### Hizmet Şartları Metnini Özelleştirme

Varsayılan olarak, onay kutusu her iki belgeye de bağlantılarla birlikte "Hizmet Şartları'nı ve Gizlilik Politikasını kabul ediyorum" metnini gösterir. Gerekirse bu metni her yerel ayar için özelleştirebilirsiniz:

1. "Metni yerel ayara göre özelleştir" seçeneğini işaretleyin
2. Açılır menüden yerel ayarı seçin ve özel metninizi girin

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-text-mode'; title='Customize TOS Text' app-screenshot-end]

### Hizmet Şartlarınızı Güncelleme

Hizmet Şartlarınızı güncellediğinizde, "Son Güncelleme" tarihini ayarlayın. Bu tarihten önce Hizmet Şartlarını kabul eden kullanıcıların tekrar kabul etmeleri gerekir:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-last-updated'; title='TOS Last Updated Date' app-screenshot-end]

### Nasıl Çalışır

- Hizmet Şartları kabul zaman damgası kullanıcı başına ve yorum başına saklanır
- Bir kullanıcı Hizmet Şartlarını kabul ettiğinde, tarih kullanıcı profilinde (kiracı başına) kaydedilir
- Kullanıcının kabul tarihinden sonra bir "Son Güncelleme" tarihi belirlerseniz, tekrar kabul etmeleri gerekir
- İzlenemeyen anonim kullanıcılar için onay kutusu her yorum gönderiminde görünür