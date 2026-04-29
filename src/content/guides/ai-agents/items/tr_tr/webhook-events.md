There are four agent webhook event types. Each event has a numeric enum value (used in payloads) and a canonical string name (used in the `event` envelope field and in the `X-FastComments-Agent-Event` HTTP header).

| Event name | Enum | Fires when |
|---|---|---|
| `trigger.succeeded` | 0 | An agent run completes with status `SUCCESS`. |
| `trigger.failed` | 1 | An agent run completes with status `ERROR`. |
| `approval.requested` | 2 | An approval is queued in `PENDING` state. |
| `approval.decided` | 3 | An approval transitions to `APPROVED`, `REJECTED`, or `EXECUTION_FAILED`. |

### `trigger.succeeded`

Ajan çalıştırması hatasız sona erdikten sonra tetiklenir. Yükün `data` alanı şunları içerir:

- `triggerId` - benzersiz çalıştırma kimliği.
- `triggerType` - çalıştırmayı başlatan [trigger reason enum](#triggers-overview).
- `status` - `SUCCESS` (string).
- `tokensUsed` - bu çalıştırmada tüketilen token sayısı.
- `wasDryRun` - ajanın [dry-run mode](#dry-run-mode) modunda olması durumunda true.
- `actions` - `TenantAgentAction` kayıtlarından oluşan dizi (bkz. [Webhook Payloads](#webhook-payloads)).
- `commentId`, `url`, `urlId` - eğer tetikleyicide bunlar varsa.

Eğer çalıştırma sıfır işlem yaptıysa, `actions` dizisi boştur - bu, başarılı bir "ajan hiçbir şey yapmamaya karar verdi" çalıştırmasıdır; bilinmesi faydalıdır.

### `trigger.failed`

Bir çalıştırma hata verdiğinde tetiklenir. Yük biçimi `trigger.succeeded` ile aynıdır; ancak `status: 'ERROR'` olur ve neyin yanlış gittiğini açıklayan ek bir `errorMessage` alanı bulunur. Olası hatalar arasında LLM çağrısı hataları, araç yürütme hataları ve çalıştırma ortasında bütçe tükenmesi sayılabilir.

`actions` yine de hatadan önce tamamlanan araç çağrılarına ait girdiler içerebilir.

### `approval.requested`

Bir onay `PENDING` durumuna alındığı anda tetiklenir. Yük şunları içerir:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status: 'PENDING'`.
- `args` - aracın LLM çağrısından **harfi harfine geçen** argümanları. Şekil araç başına değişir ve kararlı bir genel sözleşme değildir - yeni araçlar eklendikçe şema değişebilir.
- `createdAt`.
- `justification`, `confidence` - ajan bunları sağladıysa.
- `contextSnapshot` - onayın ilişkili olduğu yorum / sayfa bağlamı.

Bekleyen onayları bir chat ops kanalına iletmek için kullanışlıdır: `approval.requested` aboneliğine sahip bir Slack botu, eylemi ve gerekçeyi moderasyon kanalına anlık inceleme için gönderebilir.

### `approval.decided`

Bir onay `PENDING` dışına çıktığında tetiklenir. Yük şunları içerir:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status` - `APPROVED`, `REJECTED`, veya `EXECUTION_FAILED`.
- `decidedBy` - kararı veren moderatörün kullanıcı kimliği.
- `decidedAt` - karar verme zamanı.
- `executedAt` - ONAYLANDI ise, platformun onaylanan eylemi ne zaman yürüttüğü.
- `executionResult` - ONAYLANDI ise, yürütücünün sonucunu açıklayan bir string.
- `contextSnapshot` - yorum / sayfa bağlamı.

Bu olay tüm karar sonuçlarını kapsar:

- **Approved + executed cleanly** -> `status: APPROVED`, `executedAt` ayarlanmış, `executionResult` başarı mesajıdır.
- **Approved + executor failed** -> `status: EXECUTION_FAILED`, `executedAt` ayarlanmış, `executionResult` hatayı açıklar.
- **Rejected** -> `status: REJECTED`, `executedAt` null, `executionResult` null.

### Header

Her teslimat, olayın kanonik string adıyla (`trigger.succeeded`, vb.) bir `X-FastComments-Agent-Event` HTTP başlığı içerir. Birden fazla olay türünü işleyen tek bir URL'niz varsa bu faydalıdır.

### See also

- [Webhook Payloads](#webhook-payloads) for full per-event payload schemas.
- [Webhook Signing](#webhook-signing) for the HMAC scheme.
- [Webhook Retries](#webhook-retries) for delivery semantics.