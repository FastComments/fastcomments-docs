Um val é um receptor de webhook natural: ele tem uma URL estável, pode verificar uma assinatura e inclui SQLite e armazenamento de blobs embutidos.

FastComments assina `${timestamp}.${body}` com o segredo da API da sua conta e envia dois cabeçalhos:

[inline-code-attrs-start title = 'Cabeçalhos do webhook'; type='text' inline-code-attrs-end]
[inline-code-start]
X-FastComments-Timestamp: 1789004710        unix seconds, not milliseconds
X-FastComments-Signature: sha256=<hex>
[inline-code-end]

O método transporta o evento: **PUT** para um comentário criado ou atualizado, **DELETE** para um comentário excluído.

[inline-code-attrs-start title = 'Verificando uma entrega'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import { createHmac, timingSafeEqual } from "node:crypto";

async function receive(c) {
  // Os bytes exatos que chegaram. NÃO use c.req.json() e reserialize.
  const rawBody = await c.req.raw.text();
  const timestamp = c.req.raw.headers.get("X-FastComments-Timestamp");
  const signature = c.req.raw.headers.get("X-FastComments-Signature");

  if (!timestamp || !signature) return new Response("Missing headers", { status: 400 });

  // Rejeitar entregas antigas para que uma solicitação capturada não possa ser reproduzida mais tarde.
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

  // ...tratar JSON.parse(rawBody)
  return Response.json({ received: true });
}

app.put("/", receive);
app.delete("/", receive);
[inline-code-end]

## Duas coisas que dão dor

**Verifique os bytes brutos.** Analisar o JSON e reserializá‑lo altera a ordem das chaves e os espaços em branco, portanto o hash difere e cada entrega falha sem causa óbvia. Essa é a razão usual de um receptor de webhook "simplesmente não funciona".

**Compare em tempo constante.** Um simples `===` na assinatura vaza quantos bytes coincidem, o que é suficiente para forjar um byte de cada vez.

## Manipulando eventos

Responda rapidamente. FastComments tenta novamente em caso de resposta não‑2xx, e um endpoint que continua falhando é eventualmente desativado automaticamente, portanto faça o trabalho real após responder, em vez de inline.

Torne esse trabalho idempotente com base no ID do comentário. Uma nova tentativa é assinada novamente com um timestamp novo, e o mesmo ID de comentário chega novamente em edição e exclusão, portanto não há nada estável para desduplicar.

---