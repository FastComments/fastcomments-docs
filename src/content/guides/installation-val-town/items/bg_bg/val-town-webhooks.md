A val е естествен получател на уебхук: има стабилен URL, може да проверява подпис и има вградено SQLite и съхранение на блобове.

FastComments подписва `${timestamp}.${body}` с API тайната на вашия акаунт и изпраща две заглавки:

[inline-code-attrs-start title = 'Заглавки на уебхук'; type='text' inline-code-attrs-end]
[inline-code-start]
X-FastComments-Timestamp: 1789004710        unix seconds, not milliseconds
X-FastComments-Signature: sha256=<hex>
[inline-code-end]

Методът носи събитието: **PUT** за създаден или актуализиран коментар, **DELETE** за изтрит.

[inline-code-attrs-start title = 'Проверка на доставката'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import { createHmac, timingSafeEqual } from "node:crypto";

async function receive(c) {
  // Точните байтове, които пристигнаха. НЕ използвайте c.req.json() и повторно сериализиране.
  const rawBody = await c.req.raw.text();
  const timestamp = c.req.raw.headers.get("X-FastComments-Timestamp");
  const signature = c.req.raw.headers.get("X-FastComments-Signature");

  if (!timestamp || !signature) return new Response("Missing headers", { status: 400 });

  // Отхвърляне на остарели доставки, за да не може прихванатата заявка да бъде повторно изпълнена по-късно.
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

  // ...обработете JSON.parse(rawBody)
  return Response.json({ received: true });
}

app.put("/", receive);
app.delete("/", receive);
[inline-code-end]

## Две неща, които късат

**Проверете суровите байтове.** Парсирането на JSON и повторното му сериализиране променя реда на ключовете и whitespace, така че хешът се различава и всяка доставка се проваля без очевидна причина. Това е обичайната причина уебхук получателят „просто не работи“.

**Сравнявайте в константно време.** Обикновен `===` върху подписа разкрива колко байта съвпадат, което е достатъчно за фалшифициране по един байт наведнъж.

## Обработка на събития

Отговаряйте бързо. FastComments прави повторни опити при не‑2xx отговор, а крайна точка, която продължава да се проваля, в крайна сметка се изключва автоматично, затова извършвайте реалната работа след отговора, а не вмъкната.

Направете тази работа идемпотентна спрямо ID‑то на коментара. При повторен опит подписът се генерира отново с нов timestamp, а същото ID на коментара се получава отново при редактиране и изтриване, така че няма стабилна стойност за дедупликация.