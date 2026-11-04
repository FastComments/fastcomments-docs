每个 agent webhook 都使用您租户的 API secret 通过 HMAC-SHA256 进行签名。该签名方案与 FastComments 的评论 webhook 相同——如果您已经集成过那些 webhook，agent webhook 会复用相同的签名头和验证流程。

### 为什么需要签名

如果没有签名，知道您 webhook URL 的攻击者可以 POST 伪造的事件，使其看起来像是来自 FastComments。签名意味着您的端点可以在处理之前验证每次投递的真实性。

### 签名的工作原理

对于每次投递：

1. 平台查找该租户 + 匹配域名的 API secret（参见 [Webhooks Overview](#webhooks-overview)）。
2. 它在 `X-FastComments-Timestamp` 头中发出当前的 Unix 时间戳（毫秒）。
3. 它计算 `HMAC-SHA256(api_secret, "${timestamp}.${raw_request_body}")`（Stripe 风格），并将结果以 `sha256=<hex>` 的形式在 `X-FastComments-Signature` 头中发出。
4. 您的端点读取时间戳头，对接收到的 `${timestamp}.${body}` 重新计算 HMAC，与签名头中的 `sha256=<hex>` 值进行比较，不匹配则拒绝。

被签名的正文是平台发送的**确切字节**，前面加上 `${timestamp}.` 前缀——您的验证器必须使用原始请求正文，而不是重新序列化的 JSON 字符串（否则键顺序和空白字符会不同）。

### API secret

与[评论 webhook](/guide-webhooks.html) 使用相同的 API Secret。它按（租户，域名）划分，并在您租户的 API 设置中管理。如果您轮换了 secret，应在下一次投递之前重新部署您的验证器以读取新值。

当平台在匹配的域名上**找不到 API secret** 时，投递不会发生。webhook 日志会记录失败原因 "no API secret"。

### 验证示例（Node.js）

[inline-code-attrs-start title = 'Webhook 签名验证示例'; type='javascript' inline-code-attrs-end]
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

请使用 `timingSafeEqual` 而不是 `===`，以避免签名通过时序通道泄露。

### 签名正文的内容

完整的信封加上事件特定的 `data` 块。参见 [Webhook Payloads](#webhook-payloads)。

### 建议

- **每次投递都要验证。** 如果您的端点接受未签名的请求，您就没有任何完整性保证。
- **签名不匹配时拒绝。** 返回 401 或 403；不要在签名错误时返回 200 OK，否则您会在投递日志中掩盖攻击。
- **使用 HTTPS。** 签名保护完整性；TLS 保护机密性（包括您的 secret 和载荷中的评论文本）。
- **轮换 secret**：当有访问权限的团队成员离开时，或按计划定期轮换。

### 重放保护

仅靠签名并不能防止重放攻击——捕获了真实签名投递的攻击者可以重新发送它。重放保护由您的端点负责：

- 使用 `occurredAt` 信封字段，拒绝超过一定时间（例如 5 分钟）的投递。
- 使用 `triggerId` 或 `approvalId` 作为去重键——如果您已经处理过，则忽略重复项。

### 另请参阅

- [Webhooks Overview](#webhooks-overview)。
- [Webhook Payloads](#webhook-payloads)。
- 主要的 [Webhooks 指南](/guide-webhooks.html)，了解更广泛的签名基础设施。