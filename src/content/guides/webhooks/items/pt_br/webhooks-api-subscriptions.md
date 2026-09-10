Webhooks também podem ser gerenciados através da API REST. É assim que integrações como o Zapier se inscrevem em eventos de comentário sem tocar no painel, e segue o padrão REST Hooks: inscrever, receber eventos, cancelar inscrição.

As inscrições de API convivem ao lado dos webhooks configurados no painel. Um evento de comentário é entregue a cada webhook que corresponde ao seu domínio, cada um como sua própria entrega, independentemente de como o webhook foi criado.

## Autenticação

Cada requisição precisa da sua API Key no cabeçalho `x-api-key` (ou no parâmetro de consulta `API_KEY`) e
do seu ID de locatário no parâmetro de consulta `tenantId`. Ambos são exibidos na página API Secret no painel.

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

Retorna todos os webhooks do locatário, incluindo os gerenciados no painel (`"source": "dashboard"`).  
Filtre com `event`, `domain` ou `source`.

## Cancelar inscrição

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

Excluir uma inscrição também descarta quaisquer eventos ainda enfileirados para ela. Apenas inscrições criadas através da API podem ser excluídas dessa forma. Webhooks do painel são editados na página Webhooks.

## Payloads e assinatura

Entregas usam o mesmo payload dos webhooks do painel (veja Estruturas de Dados) e são assinadas com o mesmo esquema HMAC (veja Segurança & Tokens de API). Inscrições de API nunca recebem o cabeçalho legado `token`, portanto verifique o cabeçalho `X-FastComments-Signature` em seu lugar.

## Payloads de exemplo

```
GET https://fastcomments.com/api/v1/webhooks/sample-payloads?tenantId=YOUR_TENANT_ID&event=comment-created&limit=3
```

Retorna os comentários mais recentes da conta exatamente no formato que uma entrega carrega, permitindo que uma integração mostre dados de exemplo reais antes que o primeiro evento chegue. `event` é opcional e apenas validado, já que todo evento entrega o mesmo objeto de comentário. `limit` tem padrão 3 e aceita de 1 a 10. Custa 2 créditos de API.

```json
{
    "status": "success",
    "payloads": [
        {
            "id": "66f1c4c1e7a2b3d4f5a6b7c8",
            "urlId": "https://example.com/blog/hello-world",
            "commenterName": "Jane Reader",
            "comment": "Great article!",
            "date": "2026-09-08T12:00:00.000Z",
            "approved": true
        }
    ]
}
```

## Respondendo com 410 Gone

Se o endpoint de uma inscrição de API responder com HTTP `410 Gone`, o FastComments trata isso como um cancelamento de inscrição: a inscrição é excluída junto com seus eventos enfileirados, e nenhuma entrega adicional é tentada. Webhooks configurados no painel nunca são excluídos automaticamente; para eles um 410 é uma falha comum. Qualquer outro status de falha é reprocessado e eventualmente desabilita o webhook, conforme descrito em Como funciona & Tratamento de Repetições.

## Painel

Inscrições de API aparecem na lista de Webhooks com a origem **API**, onde um administrador pode editar, desativar, reativar ou excluí‑las.