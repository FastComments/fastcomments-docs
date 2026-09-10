Webhooks também podem ser gerenciados através da API REST. É assim que integrações como o Zapier se inscrevem em eventos de comentários sem tocar no painel, e segue o padrão REST Hooks: inscrever, receber eventos, cancelar inscrição.

As inscrições de API convivem ao lado dos webhooks configurados no painel. Um evento de comentário é entregue ao webhook do painel para seu domínio e a cada inscrição de API que corresponda, cada um como sua própria entrega. Não há limite de um assinante por evento.

## Autenticação

Cada requisição precisa da sua API Key no cabeçalho `x-api-key` (ou no parâmetro de consulta `API_KEY`) e do seu ID de locatário no parâmetro de consulta `tenantId`. Ambos são exibidos na página de Segredo da API no painel.

## Inscrever

```
POST https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
x-api-key: YOUR_API_KEY
Content-Type: application/json

{
    "url": "https://hooks.zapier.com/hooks/catch/123/abc",
    "event": "comment-created"
}
```

| Campo | Obrigatório | Descrição |
|-------|-------------|-----------|
| `url` | Sim | Um URL http ou https absoluto. |
| `event` | Sim | `comment-created`, `comment-updated` ou `comment-deleted`. |
| `domain` | Não | Um domínio da configuração da sua conta. O padrão é `*`, que recebe eventos para todos os domínios. |
| `method` | Não | `POST` (padrão), `PUT` ou `DELETE`. |

A resposta contém a inscrição:

```json
{
    "status": "success",
    "webhook": {
        "id": "66f1c4c1e7a2b3d4f5a6b7c8",
        "url": "https://hooks.zapier.com/hooks/catch/123/abc",
        "event": "comment-created",
        "domain": "*",
        "method": "POST",
        "source": "api",
        "enabled": true,
        "createdAt": "2026-09-08T12:00:00.000Z"
    }
}
```

Inscrever a mesma URL no mesmo evento e domínio novamente retorna a inscrição existente em vez de criar um duplicado, permitindo que o cliente tente novamente com segurança. Cada locatário pode ter até 50 inscrições de API.

## Listar

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

Retorna todos os webhooks do locatário, incluindo os gerenciados no painel (`"source": "dashboard"`). Filtre por `event`, `domain` ou `source`.

## Cancelar inscrição

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

Excluir uma inscrição também descarta quaisquer eventos ainda enfileirados para ela. Apenas inscrições criadas através da API podem ser excluídas desta forma. Webhooks do painel são editados na página de Webhooks.

## Cargas úteis e assinatura

As entregas usam a mesma carga útil dos webhooks do painel (veja Estruturas de Dados) e são assinadas com o mesmo esquema HMAC (veja Segurança & Tokens de API). As inscrições de API nunca recebem o cabeçalho legado `token`, portanto verifique o cabeçalho `X-FastComments-Signature`.

## Respondendo com 410 Gone

Se o endpoint de uma inscrição de API responder com HTTP `410 Gone`, o FastComments trata isso como um cancelamento de inscrição: a inscrição é excluída junto com seus eventos enfileirados, e nenhuma entrega adicional é tentada. Webhooks configurados no painel nunca são excluídos automaticamente; para eles um 410 é uma falha comum. Qualquer outro status de falha é refeito e eventualmente desabilita o webhook, conforme descrito em Como funciona & Tratamento de Repetições.

## Painel

As inscrições de API são listadas na página de Webhooks sob o domínio para o qual foram criadas, onde um administrador pode desabilitar, reabilitar ou excluí‑las.