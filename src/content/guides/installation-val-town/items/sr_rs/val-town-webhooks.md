A val је природни прималац вебхука: има стабилну URL адресу, може да верификује потпис, и има уграђену SQLite и blob складиште.

FastComments потписује `${timestamp}.${body}` тајном вашег налога за API и шаље два заглавља:

[inline-code-attrs-start title = 'Заглавља вебхука'; type='text' inline-code-attrs-end]
[inline-code-start]
X-FastComments-Timestamp: 1789004710        unix seconds, not milliseconds
X-FastComments-Signature: sha256=<hex>
[inline-code-end]

Метод преноси догађај: **PUT** за креиран или ажуриран коментар, **DELETE** за обрисан.

[inline-code-attrs-start title = 'Провера испоруке'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import { createHmac, timingSafeEqual } from "node:crypto";

async function receive(c) {
  // Тачни бајтови који су стигли. НЕ користите c.req.json() и поново серијализујте.
  const rawBody = await c.req.raw.text();
  const timestamp = c.req.raw.headers.get("X-FastComments-Timestamp");
  const signature = c.req.raw.headers.get("X-FastComments-Signature");

  if (!timestamp || !signature) return new Response("Missing headers", { status: 400 });

  // Одбаци застареле испоруке како захваћени захтев не би могао бити поново реплициран касније.
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

  // ...обради JSON.parse(rawBody)
  return Response.json({ received: true });
}

app.put("/", receive);
app.delete("/", receive);
[inline-code-end]

## Две ствари које гризу

**Провери сирове бајтове.** Парсирање JSON-а и поновно серијализовање мења редослед кључева и размаке, па хеш буде различит и свака испорука не успе без очигледног разлога. Ово је уобичајен разлог зашто прималац вебхука "само не ради".

**Упореди у константном времену.** Обичан `===` над потписом открива колико бајтова се поклапа, што је довољно за фалсификовање једног бајта појединачно.

## Обрада догађаја

Одговори брзо. FastComments поново покушава на не‑2xx одговор, а крајња тачка која стално не успева се на крају аутоматски онемогућава, па изврши прави посао након одговора уместо унутар.

Учини да рад буде идемпотентан по ID-ју коментара. Поновни покушај се поново потписује са свежим временским жигом, а исти ID коментара се поново пошиље приликом измене и брисања, тако да нема стабилног атрибута за дедаупликацију.

---