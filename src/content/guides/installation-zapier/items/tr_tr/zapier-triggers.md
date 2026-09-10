## Tetikleyiciler

Tetikleyiciler, FastComments'te bir şey olduğunda bir Zap başlatır. Üçü de anındır: FastComments, olayı gerçekleşir gerçekleşmez bir webhook aracılığıyla Zapier'a iletir. Hesabınız sorgulanmaz ve beklemek için API kredisi harcanmaz.

| Trigger | Fires when |
|---------|-----------|
| New Comment | Bir yorum gönderilir. Varsayılan olarak yalnızca onaylanmış, spam olmayan yorumlar tetiklenir. |
| Updated Comment | Bir yorum düzenlenir, onaylanır, oy alır, sabitlenir, kilitlenir veya başka bir şekilde değiştirilir. |
| Deleted Comment | Bir yorum silinir. |

Her tetikleyici tam yorumu döndürür: id, sayfa URL'si ve URL ID'si, yorum yapanın adı ve e-posta adresi, yorum metni markdown ve HTML olarak, oy sayıları, onay ve spam bayrakları, yerel ayar, domain ve tüm bahsedenler. Alanlar, Webhooks, Data Structures altında belgelenen webhook yüküyle eşleşir.

## Seçenekler

**Domain.** Her tetikleyicinin, hesabınızda yapılandırılmış domainleri listeleyen isteğe bağlı bir domain filtresi vardır. Tüm domainlerden gelen olayları almak için boş bırakın.

**Include Unapproved and Spam Comments.** Yalnızca Yeni Yorum tetikleyicisinde. Moderasyona bekletilen veya spam olarak işaretlenen yorumlar varsayılan olarak atlanır. Böyle bir yorum daha sonra onaylandığında, Güncellenmiş Yorum tetikleyicisi onun için çalışır; bu nedenle görünür hale gelen her yoruma tepki vermesi gereken bir Zap, onaylanmış alanına bir filtre uygulayarak Güncellenmiş Yorum'u kullanır.

## Teslimat Nasıl Çalışır

Bir Zap'ı açmak, hesabınızda bir webhook aboneliği oluşturur; bu, Webhooks sayfasında **API** kaynağıyla görünür. Zap'ı kapatmak ise aboneliği kaldırır. Zapier'in kendi limitleri, dakikada kabul ettiği olay sayısına uygulanır; FastComments, başarısız olan bir teslimatı artan bir gecikmeyle yeniden dener ve altı gün boyunca başarısız olan bir aboneliği devre dışı bırakır. Devre dışı bırakılan bir abonelik, Webhooks sayfasından yeniden etkinleştirilebilir veya sadece Zap'ı kapatıp tekrar açarak yeni bir abonelik oluşturulabilir.

Bir hesap en fazla 50 API aboneliği tutabilir. FastComments tetikleyicisini kullanan her Zap bir abonelik kullanır.

---