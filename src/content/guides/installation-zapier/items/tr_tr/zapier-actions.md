## İşlemler ve Aramalar

İşlemler FastComments'ta veri oluşturur; aramalar ise veriyi bulur, böylece sonraki adım bunu kullanabilir. Her işlem FastComments REST API'sini çağırır ve kendi kodunuzdan yapılan çağrıya eşdeğer API kredilerini harcar: belirtilmedikçe çağrı başına bir kredi.

## Yorum Oluştur

Bir sayfada yorum gönderir.

| Alan | Gerekli | Notlar |
|------|----------|--------|
| Sayfa URL Kimliği | Evet | Yorum widget'ının sayfada kullandığı URL kimliği. Yorumlar buna göre gruplanır. |
| Sayfa URL'si | Evet | Bildirim e-postalarında kullanılan tam sayfa URL'si. |
| Yorum | Evet | FastComments markdown'ında yorum gövdesi. |
| Yorum Yapanın Adı | Evet | İsimler e-posta başına benzersizdir, bu yüzden farklı bir e-posta ile aynı ismi kullanmak başarısız olur. |
| Yorum Yapanın E-postası | Hayır | E-posta henüz mevcut değilse, bu e-posta için bir kullanıcı oluşturulur. |
| Kullanıcı Kimliği | Hayır | Mevcut bir SSO kullanıcı kimliği. İsim ve e-posta üzerine öncelik alır. |
| Üst Yorum Kimliği | Hayır | Yanıt göndermek için ayarlayın. |
| Onaylı, Doğrulanmış | Hayır | Her ikisi de varsayılan olarak true'dur. Onaylanmamış yorumlar, denetlenene kadar gizli kalır. |
| Gönderilme Zamanı | Hayır | Varsayılan olarak şu an. |
| Avatar URL'si, Sayfa Başlığı, Yerel Ayar | Hayır | Yerel ayar varsayılan olarak `en_us`. |
| Canlı Olarak Widget'ta Göster | Hayır | Yorumu izleyicilere gerçek zamanlı olarak gönderir. 1 yerine 2 kredi maliyetlidir. |
| Spam Kontrolü Çalıştır, E-posta Gönder | Hayır | Varsayılan olarak kapalı. |

## Sayfa Oluştur

Üzerinde henüz yorum bulunmadan bir sayfa kaydı oluşturur, böylece listelenebilir ve kısıtlanabilir. URL kimliği, başlık, URL ve isteğe bağlı olarak görüntülenmesine izin verilen SSO grup kimliklerini alır.

## SSO Kullanıcı Oluştur

Tek oturum açma (single sign-on) kullanıcısı oluşturur. Kendi kullanıcı kimliğinizi, kullanıcı adınızı ve e-postanızı alır, ayrıca isteğe bağlı olarak görüntüleme adı, etiket, avatar, web sitesi, grup kimlikleri ve bildirim ve gizlilik bayraklarını alır. Yönetim rolleri Zapier'dan verilemez.

## Akış Gönderisi Oluştur

HTML içeriğinden bir FastComments akışına gönderi oluşturur, isteğe bağlı bir başlık, yazar, etiketler ve bir bağlantı önizlemesi ile.

## Etiket Oluştur

Yorumcuların kullanabileceği bir etiket oluşturur, isteğe bağlı olarak bağlandığı bir URL ile.

## Yorum İşaretle

Bir yorumu moderatör incelemesi için işaretler. İşaretleme yapan kullanıcının kimliğini sağlayın, ya da Zapier entegrasyonu olarak işaretlemek için boş bırakın.

## Aramalar

| Arama | Girdi | Döndürür |
|-------|-------|----------|
| Yorum Bul | Yorum Kimliği | Yorum, ya da hiçbir şey. |
| SSO Kullanıcı Bul | E-posta | SSO kullanıcısı, ya da hiçbir şey. |
| Sayfa Bul | URL Kimliği | Sayfa, ya da hiçbir şey. |

Hiçbir şey bulamayan bir arama Zap'i başarısız etmez. Zapier'in "bul ya da oluştur" modunda bir aramayı oluşturma ile birleştirerek eksik olduğunda sayfayı veya kullanıcıyı oluşturabilirsiniz.