每個 agent webhook 都使用您租戶的 API secret 以 HMAC-SHA256 進行簽署。此簽署機制與 FastComments 的留言 webhook 相同 - 如果您已經整合過那些功能，agent webhook 會重複使用相同的簽名標頭與驗證流程。

### 為什麼要簽署

如果沒有簽名，知道您 webhook URL 的攻擊者可以偽造看似來自 FastComments 的事件並 POST 給您。簽署可讓您的端點在處理之前驗證每次傳遞都是真實的。

### 簽名的運作方式

每次傳遞時：

1. 平台會查詢該租戶 + 匹配網域的 API secret（請參閱 [Webhooks Overview](#webhooks-overview)）。
2. 它會在 `X-FastComments-Timestamp` 標頭中發出目前的 Unix 時間戳（以毫秒為單位）。
3. 它會計算 `HMAC-SHA256(api_secret, "${timestamp}.${raw_request_body}")`（Stripe 風格），並以 `sha256=<hex>` 的形式將結果放在 `X-FastComments-Signature` 標頭中發出。
4. 您的端點讀取時間戳標頭，對其收到的 `${timestamp}.${body}` 重新計算 HMAC，與簽名標頭中的 `sha256=<hex>` 值進行比對，若不匹配則拒絕。

被簽署的內容是平台發送的**確切位元組**，並加上 `${timestamp}.` 前綴 - 您的驗證器必須使用原始請求主體，而不是重新序列化的 JSON 字串（否則鍵的順序與空白字元會不一致）。

### API secret

與[留言 webhook](/guide-webhooks.html) 使用的 API Secret 相同。它是按（租戶、網域）區分的，並在您租戶的 API 設定中管理。如果您更換（rotate）secret，應在下次傳遞之前重新部署您的驗證器以讀取新值。

當平台在匹配的網域上找不到 **API secret** 時，就不會進行傳遞。webhook 日誌會記錄失敗原因為「no API secret」。

### 驗證範例（Node.js）

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

請使用 `timingSafeEqual` 而非 `===`，以避免簽名透過時間通道洩漏。

### 簽署主體的內容

完整的信封加上事件專屬的 `data` 區塊。請參閱 [Webhook Payloads](#webhook-payloads)。

### 建議

- **每次傳遞都要驗證。** 如果您的端點接受未簽署的請求，就沒有任何完整性保證。
- **簽名不匹配時拒絕。** 回傳 401 或 403；不要在簽名錯誤時回傳 200 OK，否則您會在傳遞日誌中掩蓋攻擊行為。
- **使用 HTTPS。** 簽名保護完整性；TLS 保護機密性（包括您的 secret 以及酬載中的留言文字）。
- **更換 secrets** 當有存取權限的團隊成員離開時，或按固定排程進行。

### 重放攻擊防護

僅有簽署並不能防止重放攻擊 - 擷取到真實已簽署傳遞的攻擊者可以重新發送它。重放防護取決於您的端點：

- 使用 `occurredAt` 信封欄位，並拒絕超過（例如）5 分鐘的舊傳遞。
- 使用 `triggerId` 或 `approvalId` 作為去重鍵 - 如果您已經處理過，就忽略重複的項目。

### 另請參閱

- [Webhooks Overview](#webhooks-overview)。
- [Webhook Payloads](#webhook-payloads)。
- 主要的 [Webhooks 指南](/guide-webhooks.html)，瞭解更廣泛的簽署基礎架構。