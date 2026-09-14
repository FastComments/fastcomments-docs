A val 是一個自然的 webhook 接收器：它具有穩定的 URL、能驗證簽名，且內建 SQLite 與 blob 儲存。

FastComments 使用您帳戶的 API 密鑰對 `${timestamp}.${body}` 進行簽名，並傳送兩個標頭：

[inline-code-attrs-start title = 'Webhook 標頭'; type='text' inline-code-attrs-end]
[inline-code-start]
X-FastComments-Timestamp: 1789004710        unix seconds, not milliseconds
X-FastComments-Signature: sha256=<hex>
[inline-code-end]

此方法攜帶事件：對於新建或更新的評論使用 **PUT**，對於已刪除的評論使用 **DELETE**。

[inline-code-attrs-start title = '驗證傳遞'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import { createHmac, timingSafeEqual } from "node:crypto";

async function receive(c) {
  // 收到的精確位元組。不要使用 c.req.json() 並重新序列化。
  const rawBody = await c.req.raw.text();
  const timestamp = c.req.raw.headers.get("X-FastComments-Timestamp");
  const signature = c.req.raw.headers.get("X-FastComments-Signature");

  if (!timestamp || !signature) return new Response("Missing headers", { status: 400 });

  // 拒絕過期的傳遞，以防止捕獲的請求稍後被重放。
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

  // ...處理 JSON.parse(rawBody)
  return Response.json({ received: true });
}

app.put("/", receive);
app.delete("/", receive);
[inline-code-end]

## 兩個常見的陷阱

**驗證原始位元組。** 解析 JSON 並重新序列化會改變鍵的順序與空白，導致雜湊不同，所有傳遞都會失敗且沒有明顯原因。這通常是 webhook 接收器「根本無法運作」的原因。

**以恆定時間比較。** 對簽名使用普通的 `===` 會洩漏匹配的位元組數量，足以一次偽造一個位元組。

## 處理事件

快速回應。FastComments 會在非 2xx 回應時重試，且持續失敗的端點最終會自動被停用，因此應在回應後再執行實際工作，而非內嵌於回應中。

使其在評論 ID 上具備冪等性。重試時會使用新的時間戳重新簽名，同一評論 ID 在編輯與刪除時會再次到達，因此沒有穩定的依據可用於去重。

---