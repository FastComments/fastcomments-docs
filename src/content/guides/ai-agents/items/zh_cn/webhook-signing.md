每个代理 webhook 都使用租户的 API secret 以 HMAC-SHA256 进行签名。FastComments 的评论 webhook 使用相同的签名方案 —— 如果您已集成了那些，代理 webhook 会重用相同的签名头和验证流程。

### 为什么要签名

没有签名的话，知道您 webhook URL 的攻击者可以 POST 伪造的事件，看起来像是来自 FastComments。签名意味着您的端点可以在采取行动前验证每次投递的真实性。

### 签名如何工作

对于每次投递：

1. 平台会查找租户 + 已匹配域名的 API secret（参见 [Webhooks Overview](#webhooks-overview)）。
2. 它在 `X-FastComments-Timestamp` 头中发送以毫秒为单位的当前 Unix 时间戳。
3. 它计算 `HMAC-SHA256(api_secret, "${timestamp}.${raw_request_body}")`（类似 Stripe 的做法），并在 `X-FastComments-Signature` 头中以 `sha256=<hex>` 的形式发出结果。
4. 您的端点读取时间戳头，重新计算收到的 `${timestamp}.${body}` 的 HMAC，将其与签名头中的 `sha256=<hex>` 值进行比较，并拒绝不匹配的情况。

被签名的主体是平台发送的**精确字节**，以 `${timestamp}.` 为前缀——您的验证器必须使用原始请求体，而不是重新序列化的 JSON 字符串（否则键的顺序和空白会不同）。

### API secret

与 [comment webhooks](/guide-webhooks.html) 使用的是相同的 API Secret。它是按（租户，域）区分并在您租户的 API 设置中管理。如果您轮换该 secret，应在下一次投递之前重新部署您的验证器以读取新值。

当平台未找到匹配域的 **API secret** 时，将不会进行投递。webhook 日志会记录失败，原因是 "no API secret"。

### 验证示例 (Node.js)

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

使用 `timingSafeEqual` 而不是 `===` 来避免签名的计时通道泄露。

### 签名主体的内容

完整的 envelope 以及事件特定的 `data` 块。参见 [Webhook Payloads](#webhook-payloads)。

### 建议

- **对每次投递都进行验证。** 如果您的端点接受未签名的请求，您将无法保证完整性。
- **签名不匹配则拒绝。** 返回 401 或 403；不要在签名错误时返回 200 OK，否则您会在投递日志中掩盖攻击。
- **使用 HTTPS。** 签名保护完整性；TLS 保护机密性（包括您的 secret 和有效载荷中的评论文本）。
- **轮换 secrets** 当有访问权限的团队成员离开时，或按计划进行。

### 重放保护

仅签名本身不能防止重放攻击——捕获了真实签名投递的攻击者可以再次发送。重放保护需要由您的端点实现：

- 使用 envelope 的 `occurredAt` 字段并拒绝（例如）超过 5 分钟的投递。
- 使用 `triggerId` 或 `approvalId` 作为去重键——如果您已处理过，则忽略重复项。

### 另见

- [Webhooks Overview](#webhooks-overview)。
- [Webhook Payloads](#webhook-payloads)。
- 更广泛签名基础设施的主 [Webhooks guide](/guide-webhooks.html)。