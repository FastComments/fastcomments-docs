每個代理 webhook 都使用您租戶的 API secret 以 HMAC-SHA256 進行簽名。與 FastComments 的評論 webhook 採用相同的簽名機制 — 如果您已經整合過那些，代理 webhook 會重用相同的簽名標頭與驗證流程。

### 為什麼要簽名

如果沒有簽名，知道您 webhook URL 的攻擊者可以 POST 偽造的事件，看起來像是來自 FastComments。簽名讓您的端點在採取行動前可以驗證每次傳送的真實性。

### 簽名如何運作

對於每次傳送：

1. 平台會查找該租戶 + 匹配網域的 API secret（參見 [Webhooks Overview](#webhooks-overview)）。
2. 它在 `X-FastComments-Timestamp` 標頭中輸出當前的 Unix 時間戳（以毫秒為單位）。
3. 它計算 `HMAC-SHA256(api_secret, "${timestamp}.${raw_request_body}")`（類似 Stripe 的方式），並以 `sha256=<hex>` 的形式在 `X-FastComments-Signature` 標頭中輸出結果。
4. 您的端點讀取時間戳標頭，重新計算它收到的 `${timestamp}.${body}` 的 HMAC，將其與簽名標頭中的 `sha256=<hex>` 值比對，並拒絕不匹配的請求。

被簽名的主體是平台傳送的「精確位元組」，以 `${timestamp}.` 為前綴 — 您的驗證程式必須使用原始請求主體，而不是重新序列化的 JSON 字串（否則鍵排序與空白會不同）。

### API secret

與 [comment webhooks](/guide-webhooks.html) 使用的相同 API Secret。它是針對（租戶、網域）管理的，並在您租戶的 API 設定中管理。如果您輪換了 secret，您應該在下一次傳送之前重新部署驗證程式以讀取新值。

當平台找不到匹配網域的 **API secret** 時，傳送不會發生。webhook 日誌會記錄失敗並註明原因為 "no API secret"。

### Verification example (Node.js)

[inline-code-attrs-start title = 'Webhook 簽名驗證範例'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import crypto from 'crypto';

function verifyAgentWebhook(rawBody, signatureHeader, timestampHeader, secret) {
  const expected = 'sha256=' + crypto
    .createHmac('sha256', secret)
    .update(`${timestampHeader}.${rawBody}`)
    .digest('hex');
  return crypto.timingSafeEqual(
    Buffer.from(expected),
    Buffer.from(signatureHeader),
  );
}
[inline-code-end]

使用 `timingSafeEqual` 而不是 `===` 以避免簽名的時間通道洩漏。

### 簽名主體中包含什麼

完整的信封（envelope）以及事件特定的 `data` 區塊。參見 [Webhook Payloads](#webhook-payloads)。

### 建議

- **驗證每次傳送。** 如果您的端點接受未簽名的請求，您就沒有完整性保證。
- **於簽名不匹配時拒絕。** 回傳 401 或 403；不要在簽名錯誤時回傳 200 OK，否則您會在送達日誌中掩蓋攻擊行為。
- **使用 HTTPS。** 簽名保護完整性；TLS 保護機密性（包含您的 secret 與有效載荷中的評論文字）。
- **輪換 secrets** 當有可存取的團隊成員離開，或按照排程進行。

### 重放保護

僅簽名並不能防止重放攻擊 — 捕獲到的已簽名傳送可以被攻擊者重發。重放保護取決於您的端點：

- 使用 `occurredAt` 信封欄位並拒絕比方說超過 5 分鐘的舊傳送。
- 使用 `triggerId` 或 `approvalId` 作為去重鍵 — 如果您已處理過，就忽略重複項。

### 另見

- [Webhooks Overview](#webhooks-overview)。
- [Webhook Payloads](#webhook-payloads)。
- 有關更廣泛簽名基礎設施的主要 [Webhooks guide](/guide-webhooks.html)。