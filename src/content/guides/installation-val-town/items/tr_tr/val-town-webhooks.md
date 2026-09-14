A val, doğal bir webhook alıcısıdır: sabit bir URL'ye sahiptir, bir imzayı doğrulayabilir ve SQLite ile blob depolama yerleşiktir.

FastComments, `${timestamp}.${body}` ifadesini hesabınızın API gizli anahtarıyla imzalar ve iki başlık gönderir:

[inline-code-attrs-start title = 'Webhook başlıkları'; type='text' inline-code-attrs-end]
[inline-code-start]
X-FastComments-Timestamp: 1789004710        unix seconds, not milliseconds
X-FastComments-Signature: sha256=<hex>
[inline-code-end]

Yöntem olayı taşır: oluşturulan veya güncellenen bir yorum için **PUT**, silinen bir yorum için **DELETE**.

[inline-code-attrs-start title = 'Teslimatı doğrulama'; type='javascript' inline-code-attrs-end]
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

## Sorun yaratan iki şey

**Ham baytları doğrulayın.** JSON'u ayrıştırıp yeniden serileştirmek, anahtar sırasını ve boşlukları değiştirir, bu yüzden hash farklı olur ve her teslimat belirgin bir neden olmadan başarısız olur. Bu, bir webhook alıcısının “sadece çalışmıyor” olmasının yaygın nedenidir.

**Sabit zamanlı karşılaştırma yapın.** İmza üzerinde basit bir `===` ifadesi, kaç baytın eşleştiğini sızdırır ve bu, tek tek bir baytı taklit etmek için yeterlidir.

## Olayları işleme

Hızlı yanıt verin. FastComments, 2xx olmayan bir yanıt alındığında yeniden deneme yapar ve sürekli başarısız olan bir uç nokta sonunda otomatik olarak devre dışı bırakılır, bu yüzden gerçek işi yanıt verdikten sonra yapın, satır içinde değil.

Bu işlemi yorum kimliği üzerinde idempotent (tekrarlanabilir) hâle getirin. Bir yeniden deneme, yeni bir zaman damgası ile yeniden imzalanır ve aynı yorum kimliği düzenleme ve silme sırasında tekrar gelir, bu yüzden tutarlı bir şekilde yinelenenleri ayıklamak için sabit bir şey yoktur.