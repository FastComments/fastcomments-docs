## İşlemler ve Aramalar

İşlemler FastComments'ta veri oluşturur; aramalar ise veriyi bulur, böylece sonraki bir adımda kullanılabilir. Her işlem FastComments REST API'sini çağırır ve kendi kodunuzdan yapılan çağrı gibi aynı API kredilerini harcar: belirtilmedikçe çağrı başına bir kredi.

## Yorum Oluştur

Bir sayfada yorum gönderir.

| Alan | Gerekli | Notlar |
|------|---------|--------|
| Sayfa URL Kimliği | Evet | Yorum widget'ının sayfada kullandığı URL kimliği. Yorumlar buna göre gruplanır. |
| Sayfa URL'si | Evet | Bildirim e-postalarında kullanılan tam sayfa URL'si. |
| Yorum | Evet | FastComments markdown formatındaki yorum içeriği. |
| Yorumcu Adı | Evet | İsimler e-posta başına benzersizdir, bu yüzden farklı bir e-posta ile aynı isim kullanılamaz. |
| Yorumcu E-posta | Hayır | E-posta henüz yoksa bir kullanıcı oluşturulur. |
| Kullanıcı Kimliği | Hayır | Mevcut bir SSO kullanıcı kimliği. İsim ve e-posta üzerine öncelik alır. |
| Üst Yorum Kimliği | Hayır | Bir yanıt göndermek için ayarlanır. |
| Onaylı, Doğrulanmış | Hayır | İkisi de varsayılan olarak true'dur. Onaylanmamış yorumlar, moderasyona kadar gizli kalır. |
| Gönderildiği Tarih | Hayır | Varsayılan olarak şu an. |
| Avatar URL'si, Sayfa Başlığı, Yerel | Hayır | Yerel varsayılan olarak `en_us`'dir. |
| Widget'ta Canlı Göster | Hayır | Yorumu izleyicilere gerçek zamanlı olarak gönderir. 1 yerine 2 kredi maliyetlidir. |
| Spam Kontrolü Çalıştır, E-postaları Gönder | Hayır | Varsayılan olarak kapalıdır. |

## Sayfa Oluştur

Herhangi bir yorum mevcut olmadan önce bir sayfa kaydı oluşturur, böylece listelenebilir ve kısıtlanabilir. URL kimliği, başlık, URL ve isteğe bağlı olarak görmesini izin verilen SSO grup kimliklerini alır.

## SSO Kullanıcı Oluştur

Tek oturum açma (single sign-on) kullanıcısı oluşturur. Kendi kullanıcı kimliğinizi, kullanıcı adınızı ve e-posta adresinizi alır, ayrıca isteğe bağlı olarak görüntü adı, görüntü etiketi, avatar, web sitesi, grup kimlikleri ve bildirim ve gizlilik bayraklarını da alır. Yönetim rolleri Zapier üzerinden verilemez.

## Feed Gönderisi Oluştur

HTML içeriğinden bir FastComments akışına gönderi oluşturur. Yazar kullanıcı kimliği gereklidir (FastComments veya SSO kullanıcı kimliği); başlık, etiketler ve bir bağlantı önizlemesi isteğe bağlıdır.

## Etiket Oluştur

Yorumcuların kullanabileceği bir etiket oluşturur, isteğe bağlı bir URL'ye bağlanabilir. Etiketler hesap başına benzersizdir, bu yüzden her çalıştırmada bir etiket oluşturan bir Zap'in etikette benzersiz bir şey olması gerekir.

## Yorum Bayrakla

Bir yorumu moderatör incelemesi için işaretler. İşaretleme yapan kullanıcının kimliği gereklidir; Yorum Oluştur işleminden dönen yazar kimliği çalışır.

## Aramalar

| Arama | Girdi | Döndürür |
|-------|-------|----------|
| Yorum Bul | Yorum Kimliği | Yorum, ya da hiçbir şey. |
| SSO Kullanıcı Bul | E-posta | SSO kullanıcısı, ya da hiçbir şey. |
| Sayfa Bul | URL Kimliği | Sayfa, ya da hiçbir şey. |

Hiçbir şey bulamayan bir arama Zap'i başarısız saymaz. Zapier'in "bul ya da oluştur" modunda bir aramayı oluşturma ile birleştirerek sayfa ya da kullanıcı eksik olduğunda oluşturabilirsiniz.