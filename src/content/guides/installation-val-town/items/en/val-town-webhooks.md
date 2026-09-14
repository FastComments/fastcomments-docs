A val is a natural webhook receiver: it has a stable URL, it can verify a signature, and it has SQLite and blob storage built in.

FastComments signs `${timestamp}.${body}` with your account's API secret and sends two headers:

[inline-code-attrs-start title = 'Webhook headers'; type='text' inline-code-attrs-end]
[inline-code-start]
X-FastComments-Timestamp: 1789004710        unix seconds, not milliseconds
X-FastComments-Signature: sha256=<hex>
[inline-code-end]

The method carries the event: **PUT** for a created or updated comment, **DELETE** for a deleted one.

[inline-code-attrs-start title = 'Verifying a delivery'; type='javascript' inline-code-attrs-end]
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

## Two things that bite

**Verify the raw bytes.** Parsing the JSON and re-serializing it changes key order and whitespace, so the hash differs and every delivery fails with no obvious cause. This is the usual reason a webhook receiver "just doesn't work".

**Compare in constant time.** A plain `===` on the signature leaks how many bytes matched, which is enough to forge one byte at a time.

## Handling events

Answer quickly. FastComments retries on a non-2xx, and an endpoint that keeps failing is eventually disabled automatically, so do real work after responding rather than inline.

Make that work idempotent on the comment id. A retry is re-signed with a fresh timestamp, and the same comment id arrives again on edit and delete, so there is nothing stable to deduplicate on.
