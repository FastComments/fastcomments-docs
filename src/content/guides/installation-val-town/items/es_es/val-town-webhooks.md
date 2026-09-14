A val es un receptor de webhook natural: tiene una URL estable, puede verificar una firma y tiene SQLite y almacenamiento de blobs incorporados.

FastComments firma `${timestamp}.${body}` con el secreto API de tu cuenta y envía dos encabezados:

[inline-code-attrs-start title = 'Encabezados del webhook'; type='text' inline-code-attrs-end]
[inline-code-start]
X-FastComments-Timestamp: 1789004710        unix seconds, not milliseconds
X-FastComments-Signature: sha256=<hex>
[inline-code-end]

El método lleva el evento: **PUT** para un comentario creado o actualizado, **DELETE** para uno eliminado.

[inline-code-attrs-start title = 'Verificando una entrega'; type='javascript' inline-code-attrs-end]
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

## Dos cosas que muerden

**Verifica los bytes crudos.** Analizar el JSON y volver a serializarlo cambia el orden de las claves y los espacios en blanco, por lo que el hash difiere y cada entrega falla sin una causa evidente. Esta es la razón habitual por la que un receptor de webhook "simplemente no funciona".

**Compara en tiempo constante.** Un simple `===` sobre la firma revela cuántos bytes coinciden, lo que es suficiente para falsificar un byte a la vez.

## Manejo de eventos

Responde rápidamente. FastComments reintenta en caso de una respuesta que no sea 2xx, y un endpoint que sigue fallando se desactiva automáticamente al final, así que realiza el trabajo real después de responder en lugar de hacerlo en línea.

Haz que ese trabajo sea idempotente respecto al id del comentario. Un reintento se vuelve a firmar con una marca de tiempo nueva, y el mismo id de comentario llega de nuevo en la edición y eliminación, por lo que no hay nada estable sobre lo que deduplicar.