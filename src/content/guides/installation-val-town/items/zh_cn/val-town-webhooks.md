val 是天然的 webhook 接收器：它拥有稳定的 URL，可以验证签名，并且内置了 SQLite 和 blob 存储。

FastComments 使用您账户的 API secret 对 `${timestamp}.${body}` 进行签名，并发送两个请求头：

[inline-code-attrs-start title = 'Webhook 请求头'; type='text' inline-code-attrs-end]
[inline-code-start]
X-FastComments-Timestamp: 1789004710        unix seconds, not milliseconds
X-FastComments-Signature: sha256=<hex>
[inline-code-end]

HTTP 方法承载了事件类型：**PUT** 表示评论被创建或更新，**DELETE** 表示评论被删除。

[inline-code-attrs-start title = '验证送达'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import { createHmac, timingSafeEqual } from "node:crypto";

async function receive(c) {
  // 到达的原始字节。不要使用 c.req.json() 再重新序列化。
  const rawBody = await c.req.raw.text();
  const timestamp = c.req.raw.headers.get("X-FastComments-Timestamp");
  const signature = c.req.raw.headers.get("X-FastComments-Signature");

  if (!timestamp || !signature) return new Response("Missing headers", { status: 400 });

  // 拒绝过期的送达，防止被截获的请求在之后被重放。
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

## 两个容易踩的坑

**验证原始字节。** 解析 JSON 再重新序列化会改变键的顺序和空白字符，因此哈希值会不同，导致每次送达都失败且没有明显原因。这是 webhook 接收器“就是无法工作”的常见原因。

**以恒定时间进行比较。** 对签名使用普通的 `===` 会泄露有多少字节匹配，这足以让攻击者逐字节伪造签名。

## 处理事件

快速响应。FastComments 会在收到非 2xx 响应时重试，而持续失败的端点最终会被自动禁用，因此应在响应之后再执行实际工作，而不是内联处理。

确保该工作基于评论 id 是幂等的。重试会使用新的时间戳重新签名，并且同一条评论 id 在编辑和删除时会再次到达，因此没有稳定的字段可用于去重。