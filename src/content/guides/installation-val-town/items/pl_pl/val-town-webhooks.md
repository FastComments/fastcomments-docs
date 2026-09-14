A val jest naturalnym odbiorcą webhooków: ma stabilny URL, może weryfikować podpis i ma wbudowaną bazę SQLite oraz przechowywanie blobów.

FastComments podpisuje `${timestamp}.${body}` przy użyciu sekretu API Twojego konta i wysyła dwa nagłówki:

[inline-code-attrs-start title = 'Nagłówki webhooka'; type='text' inline-code-attrs-end]
[inline-code-start]
X-FastComments-Timestamp: 1789004710        unix seconds, not milliseconds
X-FastComments-Signature: sha256=<hex>
[inline-code-end]

Metoda przenosi zdarzenie: **PUT** dla utworzonego lub zaktualizowanego komentarza, **DELETE** dla usuniętego.

[inline-code-attrs-start title = 'Weryfikacja dostawy'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import { createHmac, timingSafeEqual } from "node:crypto";

async function receive(c) {
  // Dokładne bajty, które nadeszły. Nie używaj c.req.json() i nie serializuj ponownie.
  const rawBody = await c.req.raw.text();
  const timestamp = c.req.raw.headers.get("X-FastComments-Timestamp");
  const signature = c.req.raw.headers.get("X-FastComments-Signature");

  if (!timestamp || !signature) return new Response("Missing headers", { status: 400 });

  // Odrzuć przestarzałe dostawy, aby przechwycony request nie mógł być odtworzony później.
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

## Dwie rzeczy, które gryzą

**Zweryfikuj surowe bajty.** Parsowanie JSON i ponowne serializowanie zmienia kolejność kluczy oraz białe znaki, więc hash się różni i każda dostawa kończy się niepowodzeniem bez oczywistej przyczyny. To najczęstszy powód, dla którego odbiorca webhooka „po prostu nie działa”.

**Porównuj w stałym czasie.** Zwykłe `===` na podpisie ujawnia, ile bajtów się zgadza, co wystarczy, aby podrobić jeden bajt naraz.

## Obsługa zdarzeń

Odpowiadaj szybko. FastComments ponawia próbę przy kodzie innym niż 2xx, a endpoint, który ciągle zawodzi, zostaje ostatecznie automatycznie wyłączony, więc wykonuj rzeczywistą pracę po odpowiedzi, a nie w linii.

Uczyń to działanie idempotentnym względem identyfikatora komentarza. Ponowna próba jest podpisywana ponownie z nowym znacznikiem czasu, a ten sam identyfikator komentarza pojawia się ponownie przy edycji i usunięciu, więc nie ma nic stabilnego, na czym można by deduplikować.

---