Cada webhook de agente é assinado com HMAC-SHA256 usando o segredo de API do seu tenant. O mesmo esquema de assinatura é usado para os webhooks de comentários do FastComments - se você já integrou esses, os webhooks de agente reutilizam o mesmo cabeçalho de assinatura e fluxo de verificação.

### Por que assinar

Sem uma assinatura, um atacante que conheça a URL do seu webhook poderia enviar eventos forjados via POST que pareçam vir do FastComments. A assinatura permite que seu endpoint verifique se cada entrega é autêntica antes de agir sobre ela.

### Como as assinaturas funcionam

Para cada entrega:

1. A plataforma busca o segredo de API para o tenant + domínio correspondente (veja [Visão Geral dos Webhooks](#webhooks-overview)).
2. Ela emite o timestamp Unix atual (em milissegundos) no cabeçalho `X-FastComments-Timestamp`.
3. Ela calcula `HMAC-SHA256(api_secret, "${timestamp}.${raw_request_body}")` (estilo Stripe) e emite o resultado como `sha256=<hex>` no cabeçalho `X-FastComments-Signature`.
4. Seu endpoint lê o cabeçalho de timestamp, recalcula o HMAC sobre `${timestamp}.${body}` que recebeu, compara com o valor `sha256=<hex>` no cabeçalho de assinatura e rejeita divergências.

O corpo que é assinado são os **bytes exatos** que a plataforma enviou, prefixados com `${timestamp}.` - seu verificador deve usar o corpo bruto da requisição, não uma string JSON re-serializada (a ordem das chaves e os espaços em branco seriam diferentes).

### Segredo de API

O mesmo Segredo de API usado pelos [webhooks de comentários](/guide-webhooks.html). Ele é por (tenant, domínio) e gerenciado nas configurações de API do seu tenant. Se você rotacionar o segredo, deve reimplantar seu verificador para ler o novo valor antes da próxima entrega.

Quando a plataforma não encontra **nenhum segredo de API** para o domínio correspondente, a entrega não acontece. O log do webhook registra a falha com o motivo "no API secret".

### Exemplo de verificação (Node.js)

[inline-code-attrs-start title = 'Exemplo de Verificação de Assinatura de Webhook'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import crypto from 'crypto';

function verifyAgentWebhook(rawBody, signatureHeader, timestampHeader, secret) {
  const expected = 'sha256=' + crypto
    .createHmac('sha256', secret)
    .update(`${timestampHeader}.${rawBody}`)
    .digest('hex');
  return crypto.timingSafeEqual(
    Buffer.from(expected),
    Buffer.from(signatureHeader),
  );
}
[inline-code-end]

Use `timingSafeEqual` em vez de `===` para evitar vazamentos de canal de temporização da assinatura.

### O que está no corpo assinado

O envelope completo mais o bloco `data` específico do evento. Veja [Payloads de Webhook](#webhook-payloads).

### Recomendações

- **Verifique em cada entrega.** Se seu endpoint aceita requisições não assinadas, você não tem garantia de integridade.
- **Rejeite em caso de divergência de assinatura.** Retorne 401 ou 403; não retorne 200 OK com uma assinatura inválida, ou você mascarará ataques em seus logs de entrega.
- **Use HTTPS.** As assinaturas protegem a integridade; o TLS protege a confidencialidade (tanto o seu segredo quanto o texto dos comentários no payload).
- **Rotacione os segredos** quando membros da equipe com acesso saírem, ou em um cronograma.

### Proteção contra replay

A assinatura por si só não previne ataques de replay - um atacante que capturou uma entrega assinada real pode reenviá-la. A proteção contra replay fica a cargo do seu endpoint:

- Use o campo de envelope `occurredAt` e rejeite entregas mais antigas que, digamos, 5 minutos.
- Use o `triggerId` ou `approvalId` como chave de deduplicação - se você já o processou, ignore a duplicata.

### Veja também

- [Visão Geral dos Webhooks](#webhooks-overview).
- [Payloads de Webhook](#webhook-payloads).
- O [guia principal de Webhooks](/guide-webhooks.html) para a infraestrutura de assinatura mais ampla.