## İşlemler ve Aramalar

İşlemler FastComments'ta veri oluşturur; aramalar ise veriyi bulur, böylece sonraki adım bunu kullanabilir. Her işlem FastComments REST API'sini çağırır ve kendi kodunuzdan yapılan çağrı gibi aynı API kredilerini harcar: belirtilmedikçe her çağrı başına bir kredi.

## Yorum Oluştur

Bir sayfada yorum gönderir.

| Alan | Gerekli | Notlar |
|------|----------|--------|
| Sayfa URL Kimliği | Evet | Yorum widget'ının sayfada kullandığı URL kimliği. Yorumlar buna göre gruplanır. |
| Sayfa URL'si | Evet | Tam sayfa URL'si, bildirim e-postalarında kullanılır. |
| Yorum | Evet | FastComments markdown'ında yorum gövdesi. |
| Yorum Yapanın Adı | Evet | İsimler e-posta başına benzersizdir, bu yüzden farklı bir e-posta ile aynı ismi kullanmak başarısız olur. |
| Yorum Yapanın E-postası | Hayır | E-posta henüz mevcut değilse, bu e-posta için bir kullanıcı oluşturulur. |
| Kullanıcı Kimliği | Hayır | Mevcut bir SSO kullanıcı kimliği. İsim ve e-posta üzerine öncelik alır. |
| Üst Yorum Kimliği | Hayır | Yanıt göndermek için ayarlayın. |
| Onaylı, Doğrulanmış | Hayır | Her ikisi de varsayılan olarak true'dur. Onaylanmamış yorumlar, denetlenene kadar gizli kalır. |
| Gönderilme Zamanı | Hayır | Varsayılan olarak şu an. |
| Avatar URL'si, Sayfa Başlığı, Yerel Ayar | Hayır | Yerel ayar varsayılan olarak `en_us`'tir. |
| Widget'ta Canlı Göster | Hayır | Yorumu izleyicilere gerçek zamanlı olarak gönderir. 1 yerine 2 kredi maliyetlidir. |
| Spam Kontrolü Çalıştır, E-posta Gönder | Hayır | Varsayılan olarak kapalı. |

## Sayfa Oluştur veya Güncelle

Herhangi bir yorum mevcut olmadan önce bir sayfa kaydı oluşturur, böylece listelenebilir ve kısıtlanabilir. URL kimliği, başlık, URL ve isteğe bağlı olarak görüntüleyebilecek SSO grup kimliklerini alır. Aynı URL kimliğine sahip bir sayfa zaten varsa, verilen alanlarla güncellenir, böylece bir Zap aynı sayfa için tekrar tekrar çalışabilir.

## SSO Kullanıcısı Oluştur veya Güncelle

Tek oturum açma (single sign-on) kullanıcısı oluşturur. Kendi kullanıcı kimliğinizi, kullanıcı adınızı ve e-postanızı alır, ayrıca isteğe bağlı olarak gösterim adı, gösterim etiketi, avatar, web sitesi, grup kimlikleri ve bildirim ve gizlilik bayraklarını alır. Aynı kimliğe sahip bir kullanıcı zaten varsa, bunun yerine güncellenir. Yönetim rolleri Zapier'dan verilemez.

## Akış Gönderisi Oluştur

HTML içeriğinden bir FastComments akışına gönderi oluşturur. Yazar kullanıcı kimliği gereklidir (FastComments veya SSO kullanıcı kimliği); başlık, etiketler ve bir bağlantı önizlemesi isteğe bağlıdır.

## Etiket Oluştur veya Güncelle

Yorumcuların kullanabileceği bir etiket oluşturur, isteğe bağlı olarak bağlandığı bir URL ile. Etiket zaten varsa, bunun yerine güncellenir.

## Yorum İşaretle

Bir yorumu moderatör incelemesi için işaretler. İşaretleme yapan kullanıcının kimliği gereklidir; Yorum Oluştur tarafından döndürülen yazar kimliği çalışır.

## Aramalar

| Arama | Girdi | Döndürür |
|-------|-------|----------|
| Yorum Bul | Yorum Kimliği | Yorum, ya da hiçbir şey. |
| SSO Kullanıcısı Bul | E-posta | SSO kullanıcısı, ya da hiçbir şey. |
| Sayfa Bul | URL Kimliği | Sayfa, ya da hiçbir şey. |

Hiçbir şey bulamayan bir arama Zap'i başarısız etmez. SSO Kullanıcısı Bul ve Sayfa Bul, Zapier'ın "varsa oluşturma" seçeneğini sunar; hiçbir şey bulunmadığında eşleşen oluşturma işlemini çalıştırır.