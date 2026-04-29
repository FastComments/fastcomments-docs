---
Bir **onay** (approval), platformun yürütmeden önce bir insanın onaylamasını veya reddetmesini gerektiren sıraya alınmış bir araç çağrısıdır.

### Onayların yapılandırılması

Ajan düzenleme formunda, **Approvals** bölümü ajanın çağırmasına izin verilen her aracı (izin listesi) listeler ve insan tarafından incelenmesi gereken araçları işaretlemenize izin verir. İşaretlenmemiş araçlar hemen yürütülür. İşaretlenmiş araçlar sıraya alınır.

İzin verilmeyen araçlar *doğrudan reddedilir*, sıraya alınmaz - onaylar yalnızca aksi halde izin verilen araçlara uygulanır.

### Engellenmiş bir eylem tetiklendiğinde neler olur

1. Ajan bir araç çağrısı seçer (ör. `ban_user`) argümanlar, gerekçe ve güven ile.
2. Yürütmek yerine platform, araç adı, argümanlar, gerekçe, güven ve onu üreten tetikleyiciyi tanımlayan bir bağlam anlık görüntüsü ile `PENDING` durumunda bir onay sıraya alır.
3. Bildirimler değerlendiricilere gider (bkz. [Onay Bildirimleri](#approval-notifications)).
4. Ajanın çalışması tamamlanır ve kaydedilir - eylem [Run Detail View](#run-detail-view) içinde **Onay Bekliyor** olarak gösterilir.

### Onayları inceleme

[my-account/ai-agent-approvals](https://fastcomments.com/auth/my-account/ai-agent-approvals) adresindeki onay gelen kutusu bekleyen, onaylanmış, reddedilmiş ve yürütme hatası olan onayları listeler. Her biri için:

- **Araç adı ve argümanlar** - ajanın tam olarak yapmak istediği şey.
- **Ajanın gerekçesi** - ajanın sağladığı gerekçe.
- **Güven** - ajanın kendi değerlendirdiği güven düzeyi, 0.0 ile 1.0 arasında.
- **Bağlam anlık görüntüsü** - eylemin hedeflediği yorum, sayfa ve kullanıcı.

İki düğme: **Approve** ve **Reject**. Approve aslında aracı yürütür; Reject atığı siler.

### Onay durumu (status) durumları

Bir onay bu durumlar arasında ilerler:

| State | Meaning |
|---|---|
| `PENDING` | İnsan kararı bekleniyor. |
| `APPROVED` | Bir insan onayladı ve eylem çalıştı. |
| `REJECTED` | Bir insan reddetti. Eylem çalışmadı. |
| `EXECUTION_FAILED` | Bir insan onayladı ancak eylem başarısız oldu (ör. hedef yorum zaten silinmişti). |
| `EXECUTING` | Geçici: bir insan Approve'a tıkladı ve eylem çalışıyor. Gerçek yan etkisi olan bir aracın iki kez çalışmaması için eşzamanlı approve tıklamalarını sıralamak için kullanılır. |

`EXECUTING` durumu, iki değerlendirici aynı anda Approve'a tıkladığında önemlidir - biri kazanır, diğer onayın zaten ilerlemiş olduğunu görür.

### Neler temizlenir

Bekleyen onaylar karara kadar bekler. Oluşturulmasından itibaren **90 gün** sonra otomatik olarak süresi dolar. Diğer herhangi bir durumda olan onaylar da depolama temizliği için 90 gün sonra temizlenir.

Onayın "decided by" / "decided at" / "executed at" / "execution result" alanları onay durumlar arasında ilerledikçe doldurulur - tümü gelen kutusu detay görünümünde görünür.

### Webhook entegrasyonu

Onaylar ilerledikçe iki webhook olayı tetiklenir:

- **`approval.requested`** - `PENDING` eklendiğinde.
- **`approval.decided`** - `APPROVED`, `REJECTED` veya `EXECUTION_FAILED` durumuna geçildiğinde.

Her ikisi de diğer tüm webhook'lar gibi imzalanır. Bakınız [Webhook Events](#webhook-events) ve [Webhook Payloads](#webhook-payloads).

### Güçlendirme: bilinen araç kısıtı

Onaylar, tanınmış bir ajan aracı olmayan herhangi bir araç adının sıraya alınmasını reddeder. Bu derinlemesine bir savunma kontrolüdür: gelecekteki bir kod yolu bir LLM kaynaklı araç adını onay akışına geçirirse bile, o dize asla sıraya alınmış bir onay olarak yer alamaz. Bilinen araç adları kümesi [Tools Reference](#tools-overview) bölümünde listelenen aynı kümedir.

### Yaygın kısıtlama kalıpları

- **Yepyeni moderasyon ajanı** - `ban_user`, `mark_comment_spam`, `mark_comment_approved`, `send_email` için kısıt uygulayın. Birkaç hafta gelen kutusunu izleyin, ardından düşük hata oranlı araçların kısıtlamasını kaldırın.
- **Uzun vadeli moderasyon ajanı** - `ban_user` ve geri dönüşü olmayan herhangi bir eylem (`deleteAllUsersComments`, `banIP`) için kısıtı sonsuza dek açık tutun.
- **AB bölgesi** - `ban_user` Madde 17 gereği ne işaretleseniz işaretlemeyin kilitlenmiştir. Bakınız [AB DSA Madde 17 Uygunluğu](#eu-dsa-compliance).

### Onayların yapmadıkları

- Ajanın diğer araç çağrılarını geciktirmezler. Ajanın çalışması birkaç araç çağırabilir ve yalnızca kısıtlanmış olanlar sıraya alınır - geri kalan normal şekilde yürütülür.
- İnsan reddederse ajanın çalışmasını geri almazlar. Çalışmanın kısıtlanmamış kısmı zaten tamamlanmıştır.

---