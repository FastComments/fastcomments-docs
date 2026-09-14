Ein val ist ein natürlicher Webhook‑Empfänger: Er hat eine stabile URL, kann eine Signatur verifizieren und hat SQLite‑ und Blob‑Speicher integriert.

FastComments signiert `${timestamp}.${body}` mit dem API‑Geheimnis Ihres Kontos und sendet zwei Header:

[inline-code-attrs-start title = 'Webhook-Header'; type='text' inline-code-attrs-end]
[inline-code-start]
X-FastComments-Timestamp: 1789004710        unix seconds, not milliseconds
X-FastComments-Signature: sha256=<hex>
[inline-code-end]

Die Methode überträgt das Ereignis: **PUT** für einen erstellten oder aktualisierten Kommentar, **DELETE** für einen gelöschten.

[inline-code-attrs-start title = 'Verifizierung einer Zustellung'; type='javascript' inline-code-attrs-end]
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

## Zwei Dinge, die Probleme verursachen

**Verifiziere die rohen Bytes.** Das Parsen des JSON und das erneute Serialisieren ändert die Schlüsselreihenfolge und Leerzeichen, sodass der Hash abweicht und jede Zustellung fehlschlägt, ohne offensichtlichen Grund. Das ist der übliche Grund, warum ein Webhook‑Empfänger „einfach nicht funktioniert“.

**Vergleiche in konstanter Zeit.** Ein einfaches `===` auf die Signatur leckt, wie viele Bytes übereinstimmen, was ausreicht, um Byte für Byte zu fälschen.

## Ereignisse verarbeiten

Antworte schnell. FastComments versucht es bei einem Nicht‑2xx‑Status erneut, und ein Endpunkt, der ständig fehlschlägt, wird schließlich automatisch deaktiviert. Führe daher die eigentliche Arbeit erst nach der Antwort aus, nicht inline.

Stelle sicher, dass die Verarbeitung anhand der Kommentar‑ID idempotent ist. Ein Retry wird mit einem neuen Zeitstempel erneut signiert, und dieselbe Kommentar‑ID kommt bei Bearbeitung und Löschung erneut, sodass es nichts Stabiles zum Deduplizieren gibt.

---