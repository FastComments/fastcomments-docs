## Örnek Zaps

Kurulumu dakikalar içinde tamamlanabilen birkaç iş akışı.

**Yeni yorumlar hakkında bildirim alın.** Yeni Yorum, ardından Slack "Send Channel Message" veya Discord "Send Channel Message". Yorumcunun adını, yorum metnini ve sayfa URL'sini mesaja eşleyin. Site başına farklı bir kanal bildirmek için domain filtresini ekleyin.

**Her yorumun kaydını tutun.** Yeni Yorum, ardından Google Sheets "Create Spreadsheet Row". Silinen Yorum'u ikinci bir Zap olarak ekleyin; bu, yorum kimliğiyle bir satır ekler, böylece tablo bir denetim izi olarak da hizmet eder.

**Bir yorum onaylandığında yazara e-posta gönderin.** Güncellenen Yorum, onaylandığında Zapier filtresiyle birlikte, ardından Gmail "Send Email". Güncellenen Yorum her değişiklikte tetiklendiği için, filtre bu Zap'in yalnızca onaylarda çalışmasını sağlar.

**Yorumcuları CRM'inize veya e-posta listesine ekleyin.** Yeni Yorum, ardından HubSpot "Create or Update Contact" veya Mailchimp "Add or Update Subscriber" yorumcunun e-posta adresini kullanarak. Herhangi birini pazarlama listesine eklemeden önce gizlilik politikanıza ve yerel yasalara saygı gösterin.

**Bir formdan yorum oluşturun.** Typeform veya Google Forms "New Response", ardından FastComments "Create Comment" ile sitenizin referanslar için kullandığı sayfa URL kimliğini kullanın. Görünmeden önce her birini incelemek için Onaylandı seçeneğini işaretlemeyin.

**Duyuruları bir akışa gönderin.** RSS by Zapier "New Item in Feed", ardından öğenin başlığı, içeriği ve bağlantısı ile Feed Gönderisi Oluştur.

**Üyeleri SSO kullanıcıları olarak sağlayın.** Memberstack, Memberful veya kendi webhook'unuz, ardından "find or create" modunda SSO Kullanıcısını Bul ve ardından SSO Kullanıcısını Oluştur.

**Raporlanan yorumları yükseltin.** Güncellenen Yorum, sıfırın üzerindeki bayrak sayısına göre filtrelenir, ardından Trello "Create Card" veya Linear "Create Issue" ile yorum kimliği ve moderasyon sayfasına bir bağlantı eklenir.

**Sayfaları yayına girdiğinde yayınlayın.** WordPress veya Ghost "New Post", ardından gönderi URL'si ile Sayfa Oluştur, böylece sayfa ilk yorumdan önce listelenir ve kısıtlanır.

**Silinen yorumları arşivleyin.** Silinen Yorum, ardından uyumluluk saklaması için tam yorumla Airtable "Create Record".