---
`localhost` için üretim gibi aynı adımları izleyin. Üretim domain'leri ve API Secrets ayarlandığından emin olun.

İlk olarak, [Webhooks admin](https://fastcomments.com/auth/my-account/manage-data/webhooks) adresine gidin. Bu, Manage Data -> Webhooks üzerinden erişilebilir.

Sayfa, hesabınızdaki tüm webhook'ları listeler:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; alt='Webhooks yönetim sayfası, her webhook\'u URL\'si, olayı, domain\'i, yöntemi, durumu ve kuyruğa alınan olay sayısı ile listeler'; title='Webhooks Listesi'; cacheBuster = 'v4' app-screenshot-end]

**New Webhook**'a tıklayarak bir tane ekleyin. Her webhook'un bir URL'si, bir yorum olayı (oluşturuldu, güncellendi veya silindi), bir domain'i ve bir HTTP yöntemi vardır:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks/new'; selector = '.content'; alt='Yeni webhook formu, URL, olay, domain ve HTTP yöntemi alanları ile Test Yükü Gönder butonu içerir'; title='Yeni Webhook'; cacheBuster = 'v4' app-screenshot-end]

Her webhook bağımsız olarak teslim edilir. Aynı olayı birden fazla uç noktaya gönderebilirsiniz ve **All Domains** kapsamına sahip bir webhook, aynı olay için domain'e özgü bir webhook mevcut olsa bile tüm domain'lerden yorum alır. Aynı URL, olay ve domain iki kez eklenemez.

Kaydetmeden önce, uç noktanın imzalı bir isteği kabul edip etmediğini kontrol etmek için **Send Test Payload**'a tıklayın. Ayrıntılar için bir sonraki bölüm olan "Testing"e bakın.

Listeden bir webhook'u düzenleyebilir, devre dışı bırakabilir, yeniden etkinleştirebilir veya silebilirsiniz. Devre dışı bırakma, webhook yeniden etkinleştirilene kadar kuyruğa alınan olayları tutar; silme ise bunları yok eder.

Webhook'lar ayrıca API üzerinden, örneğin Zapier ile oluşturulabilir. Bunlar aynı listede **API** kaynağıyla görünür. API üzerinden Webhook Yönetimine bakın.
---