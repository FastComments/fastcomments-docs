A val — це природний отримувач вебхуків: він має стабільний URL, може перевіряти підпис і має вбудовану SQLite та сховище блобів.

FastComments підписує `${timestamp}.${body}` за допомогою API‑секрету вашого облікового запису і надсилає два заголовки:

[inline-code-attrs-start title = 'Заголовки вебхука'; type='text' inline-code-attrs-end]
[inline-code-start]
X-FastComments-Timestamp: 1789004710        unix seconds, not milliseconds
X-FastComments-Signature: sha256=<hex>
[inline-code-end]

Метод передає подію: **PUT** для створеного або оновленого коментаря, **DELETE** для видаленого.

[inline-code-attrs-start title = 'Перевірка доставки'; type='javascript' inline-code-attrs-end]
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

## Дві речі, які можуть зіпсувати

**Перевірте сирі байти.** Парсинг JSON та його повторна серіалізація змінює порядок ключів і пробіли, тому хеш відрізняється і кожна доставка провалюється без очевидної причини. Це звичайна причина, чому отримувач вебхука «просто не працює».

**Порівнюйте за постійний час.** Проста операція `===` над підписом розкриває, скільки байтів збіглося, чого достатньо, щоб підробити один байт за раз.

## Обробка подій

Відповідайте швидко. FastComments повторює запит у випадку відповіді, відмінної від 2xx, і кінцева точка, яка постійно помиляється, зрештою автоматично вимикається, тому реальну роботу виконуйте після відповіді, а не вбудовано.

Зробіть цю роботу ідемпотентною за ідентифікатором коментаря. Повторна спроба підписується новим часовим міткою, і той самий ідентифікатор коментаря надходить знову при редагуванні та видаленні, тому немає стабільного критерію для дедуплікації.