Dört adet ajan webhook olay türü vardır. Her olayın sayısal bir enum değeri (payload'larda kullanılır) ve kanonik bir dize adı vardır (`event` zarf alanında ve `X-FastComments-Agent-Event` HTTP başlığında kullanılır).

| Olay adı | Enum | Ne zaman tetiklenir |
|---|---|---|
| `trigger.succeeded` | 0 | Bir ajan çalıştırması `SUCCESS` durumuyla tamamlandığında. |
| `trigger.failed` | 1 | Bir ajan çalıştırması `ERROR` durumuyla tamamlandığında. |
| `approval.requested` | 2 | Bir onay `PENDING` durumunda kuyruğa alındığında. |
| `approval.decided` | 3 | Bir onay `APPROVED`, `REJECTED` veya `EXECUTION_FAILED` durumuna geçtiğinde. |

### `trigger.succeeded`

Ajanın çalıştırması hatasız bittikten sonra tetiklenir. Payload'ın `data` alanı şunları içerir:

- `triggerId` - benzersiz çalıştırma ID'si.
- `triggerType` - çalıştırmayı başlatan [tetikleme nedeni enum'u](#triggers-overview).
- `status` - `SUCCESS` (dize).
- `tokensUsed` - bu çalıştırmada tüketilen token sayısı.
- `wasDryRun` - ajan [dry-run modunda](#dry-run-mode) idiyse true.
- `actions` - `TenantAgentAction` kayıtları dizisi (bkz. [Webhook Payloads](#webhook-payloads)).
- `commentId`, `url`, `urlId` - tetiklemede varsa.

Çalıştırma hiçbir eylem almadıysa, `actions` dizisi boştur - bu, "ajan hiçbir şey yapmaya karar verdi" anlamına gelen başarılı bir çalıştırmadır ve bunu bilmek faydalıdır.

### `trigger.failed`

Bir çalıştırma hata verdiğinde tetiklenir. `trigger.succeeded` ile aynı payload yapısına sahiptir, ancak `status: 'ERROR'` ve neyin yanlış gittiğini açıklayan ek bir `errorMessage` alanı bulunur. Olası hatalar arasında LLM çağrısı başarısızlıkları, araç gönderim (dispatch) başarısızlıkları ve çalıştırma ortasında bütçe tükenmesi yer alır.

`actions`, hata gerçekleşmeden önce tamamlanan araç çağrıları için girişler içerebilir.

### `approval.requested`

Bir onay `PENDING` durumunda kuyruğa alındığı anda tetiklenir. Payload şunları içerir:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status: 'PENDING'`.
- `args` - LLM çağrısından **birebir geçirilen** aracın argümanları. Yapı araca göre değişir ve kararlı bir genel sözleşme değildir - yeni araçlar eklendikçe şema değişebilir.
- `createdAt`.
- `justification`, `confidence` - ajan bunları sağladıysa.
- `contextSnapshot` - onayın ilişkili olduğu yorum / sayfa bağlamı.

Bekleyen onayları bir chat ops kanalına iletmek için faydalıdır: `approval.requested` olayına abone olan bir Slack botu, eylemi ve gerekçeyi bir moderasyon kanalına göndererek tek bakışta inceleme sağlayabilir.

### `approval.decided`

Bir onay `PENDING` durumundan çıktığında tetiklenir. Payload şunları içerir:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status` - `APPROVED`, `REJECTED` veya `EXECUTION_FAILED`.
- `decidedBy` - kararı veren moderatörün kullanıcı ID'si.
- `decidedAt` - kararın verildiği zaman.
- `executedAt` - APPROVED ise, platformun onaylanan eylemi gerçekleştirdiği zaman.
- `executionResult` - APPROVED ise, gerçekleştiricinin sonucunu açıklayan bir dize.
- `contextSnapshot` - yorum / sayfa bağlamı.

Bu olay tüm karar sonuçlarını kapsar:

- **Onaylandı + sorunsuz gerçekleştirildi** -> `status: APPROVED`, `executedAt` ayarlı, `executionResult` başarı mesajıdır.
- **Onaylandı + gerçekleştirici başarısız oldu** -> `status: EXECUTION_FAILED`, `executedAt` ayarlı, `executionResult` başarısızlığı açıklar.
- **Reddedildi** -> `status: REJECTED`, `executedAt` null, `executionResult` null.

### Header

Her teslimat, olayın kanonik dize adını (`trigger.succeeded`, vb.) içeren bir `X-FastComments-Agent-Event` HTTP başlığı içerir. Uç noktanız birden fazla olay türünü işleyen tek bir URL ise faydalıdır.

### See also

- Olay başına tam payload şemaları için [Webhook Payloads](#webhook-payloads).
- HMAC şeması için [Webhook Signing](#webhook-signing).
- Teslimat semantiği için [Webhook Retries](#webhook-retries).