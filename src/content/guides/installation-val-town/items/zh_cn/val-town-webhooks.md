A val 是一种天然的 webhook 接收器：它拥有稳定的 URL，能够验证签名，并内置 SQLite 和 blob 存储。

FastComments signs `${timestamp}.${body}` with your account's API secret and sends two headers:

[inline-code-attrs-start title = 'Webhook 标头'; type='text' inline-code-attrs-end]
[inline-code-start]
X-FastComments-Timestamp: 1789004710        unix seconds, not milliseconds
X-FastComments-Signature: sha256=<hex>
[inline-code-end]

The method carries the event: **PUT** for a created or updated comment, **DELETE** for a deleted one.

[inline-code-attrs-start title = '验证交付'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import { createHmac, timingSafeEqual } from "node:crypto";

async function receive(c) {
  // 到达的原始字节。不要使用 c.req.json() 并重新序列化。
  const rawBody = await c.req.raw.text();
  const timestamp = c.req.raw.headers.get("X-FastComments-Timestamp");
  const signature = c.req.raw.headers.get("X-FastComments-Signature");

  if (!timestamp || !signature) return new Response("Missing headers", { status: 400 });

  // 拒绝过期的交付，以防捕获的请求被稍后重放。
  if (Math.abs(Math.floor(Date.now() / 1000) - Number(timestamp)) > 300) {
    return new Response("Timestamp outside window", { status: 400 });
  }

  const expected = "sha256=" + createHmac("sha256", Deno.env.get("FASTCOMMENTS_API_SECRET"))
    .update(`${timestamp}.${rawBody}`)
    .digest("hex");

  const a = new TextEncoder().encode(signature);
  const b = new TextEncoder().encode(expected);
  if (a.length !== b.length || !timingSafeEqual(a, b)) {
    return new Response("Signature mismatch", { status: 401 });
  }

  // ...处理 JSON.parse(rawBody)
  return Response.json({ received: true });
}

app.put("/", receive);
app.delete("/", receive);
[inline-code-end]

## 两个常见问题

**验证原始字节。** 解析 JSON 并重新序列化会改变键的顺序和空白字符，导致哈希不同，所有交付都会因未知原因失败。这是 webhook 接收器“根本不起作用”的常见原因。

**常量时间比较。** 对签名使用普通的 `===` 会泄露匹配的字节数，这足以一次伪造一个字节。

## 处理事件

快速响应。FastComments 会在非 2xx 响应时重试，且持续失败的端点会被自动禁用，因此应在响应后再进行实际工作，而不是在响应时内联处理。

使处理在评论 ID 上具备幂等性。重试时会使用新的时间戳重新签名，同一评论 ID 在编辑和删除时会再次到达，因此没有稳定的字段可用于去重。

---