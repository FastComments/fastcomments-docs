A val — это естественный получатель веб‑хуков: у него стабильный URL, он может проверять подпись и имеет встроенные SQLite и блоб‑хранилище.

FastComments подписывает `${timestamp}.${body}` с помощью API‑секрета вашего аккаунта и отправляет два заголовка:

[inline-code-attrs-start title = 'Заголовки вебхука'; type='text' inline-code-attrs-end]
[inline-code-start]
X-FastComments-Timestamp: 1789004710        unix seconds, not milliseconds
X-FastComments-Signature: sha256=<hex>
[inline-code-end]

Метод передаёт событие: **PUT** для созданного или обновлённого комментария, **DELETE** для удалённого.

[inline-code-attrs-start title = 'Проверка доставки'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import { createHmac, timingSafeEqual } from "node:crypto";

async function receive(c) {
  // Точные байты, которые пришли. НЕ используйте c.req.json() и повторную сериализацию.
  const rawBody = await c.req.raw.text();
  const timestamp = c.req.raw.headers.get("X-FastComments-Timestamp");
  const signature = c.req.raw.headers.get("X-FastComments-Signature");

  if (!timestamp || !signature) return new Response("Missing headers", { status: 400 });

  // Отклоняем устаревшие доставки, чтобы перехваченный запрос нельзя было воспроизвести позже.
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

## Два момента, которые могут вызвать проблемы

**Проверяйте сырые байты.** Парсинг JSON и повторная сериализация меняют порядок ключей и пробелы, поэтому хеш отличается и каждая доставка завершается ошибкой без очевидной причины. Это обычная причина, почему получатель веб‑хука «просто не работает».

**Сравнивайте за постоянное время.** Обычное `===` по подписи раскрывает, сколько байтов совпало, чего достаточно, чтобы подделать один байт за раз.

## Обработка событий

Отвечайте быстро. FastComments повторяет запрос при ответе, не являющемся 2xx, и конечная точка, постоянно вызывающая ошибки, в конце концов автоматически отключается, поэтому выполняйте реальную работу после отправки ответа, а не внутри него.

Сделайте работу идемпотентной по ID комментария. При повторе запрос подписывается заново с новым timestamp, и тот же ID комментария приходит снова при редактировании и удалении, поэтому нет стабильного признака для дедупликации.