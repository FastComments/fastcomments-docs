A val は自然な Webhook 受信者です。安定した URL を持ち、署名を検証でき、SQLite と BLOB ストレージが組み込まれています。

FastComments は `${timestamp}.${body}` にアカウントの API シークレットで署名し、2 つのヘッダーを送信します。

[inline-code-attrs-start title = 'Webhook ヘッダー'; type='text' inline-code-attrs-end]
[inline-code-start]
X-FastComments-Timestamp: 1789004710        unix seconds, not milliseconds
X-FastComments-Signature: sha256=<hex>
[inline-code-end]

このメソッドはイベントを伝えます。作成または更新されたコメントには **PUT**、削除されたコメントには **DELETE** を使用します。

[inline-code-attrs-start title = '配信の検証'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import { createHmac, timingSafeEqual } from "node:crypto";

async function receive(c) {
  // The exact bytes that arrived. Do NOT use c.req.json() and re-serialize.
  const rawBody = await c.req.raw.text();
  const timestamp = c.req.raw.headers.get("X-FastComments-Timestamp");
  const signature = c.req.raw.headers.get("X-FastComments-Signature");

  if (!timestamp || !signature) return new Response("Missing headers", { status: 400 });

  // Reject stale deliveries so a captured request cannot be replayed later.
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

  // ...handle JSON.parse(rawBody)
  return Response.json({ received: true });
}

app.put("/", receive);
app.delete("/", receive);
[inline-code-end]

## 問題となる2つの点

**生バイトを検証する。** JSON を解析して再シリアライズするとキーの順序や空白が変わり、ハッシュが一致せず、明確な原因なしにすべての配信が失敗します。これが Webhook 受信者が「うまく動かない」一般的な理由です。

**一定時間で比較する。** 署名に対して単純な `===` を使用すると、何バイト一致したかが漏洩し、1 バイトずつ偽造するのに十分です。

## イベントの処理

迅速に応答してください。FastComments は 2xx 以外のステータスで再試行し、失敗が続くエンドポイントは最終的に自動で無効化されます。そのため、インラインで処理せずにレスポンスを返した後に実際の作業を行ってください。

コメント ID に対して冪等に動作させてください。再試行時には新しいタイムスタンプで再署名され、編集や削除時に同じコメント ID が再度届くため、重複排除できる安定した情報はありません。

---