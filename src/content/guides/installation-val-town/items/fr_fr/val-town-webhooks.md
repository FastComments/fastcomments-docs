A val est un récepteur de webhook naturel : il possède une URL stable, il peut vérifier une signature, et il intègre SQLite et le stockage de blobs.

FastComments signe `${timestamp}.${body}` avec le secret API de votre compte et envoie deux en‑têtes :

[inline-code-attrs-start title = 'En‑têtes du webhook'; type='text' inline-code-attrs-end]
[inline-code-start]
X-FastComments-Timestamp: 1789004710        unix seconds, not milliseconds
X-FastComments-Signature: sha256=<hex>
[inline-code-end]

La méthode transporte l'événement : **PUT** pour un commentaire créé ou mis à jour, **DELETE** pour un commentaire supprimé.

[inline-code-attrs-start title = 'Vérification d\'une livraison'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import { createHmac, timingSafeEqual } from "node:crypto";

async function receive(c) {
  // Les octets exacts reçus. N'utilisez PAS c.req.json() et ne les re‑sérialisez pas.
  const rawBody = await c.req.raw.text();
  const timestamp = c.req.raw.headers.get("X-FastComments-Timestamp");
  const signature = c.req.raw.headers.get("X-FastComments-Signature");

  if (!timestamp || !signature) return new Response("Missing headers", { status: 400 });

  // Rejeter les livraisons obsolètes afin qu'une requête capturée ne puisse pas être rejouée plus tard.
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

  // ...traiter JSON.parse(rawBody)
  return Response.json({ received: true });
}

app.put("/", receive);
app.delete("/", receive);
[inline-code-end]

## Deux choses qui posent problème

**Vérifier les octets bruts.** Analyser le JSON et le re‑sérialiser modifie l'ordre des clés et les espaces, ainsi le hachage diffère et chaque livraison échoue sans cause évidente. C’est la raison habituelle pour laquelle un récepteur de webhook « ne fonctionne tout simplement pas ».

**Comparer en temps constant.** Un simple `===` sur la signature révèle le nombre d'octets correspondants, ce qui suffit à falsifier un octet à la fois.

## Gestion des événements

Répondez rapidement. FastComments réessaye en cas de réponse non‑2xx, et un point de terminaison qui échoue continuellement est finalement désactivé automatiquement, il faut donc effectuer le vrai travail après avoir répondu plutôt qu’en ligne.

Rendez cela idempotent sur l'ID du commentaire. Un nouveau essai est re‑signé avec un nouveau horodatage, et le même ID de commentaire arrive de nouveau lors d'une modification ou d'une suppression, il n’y a donc rien de stable sur quoi dédupliquer.

---