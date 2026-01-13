Um objeto `PendingWebhookEvent` representa um evento de webhook enfileirado que está pendente.

Objetos `PendingWebhookEvent` são criados automaticamente e não podem ser criados manualmente via API. Eles também expiram após um ano.
Eles podem ser excluídos, o que remove a tarefa da fila.

Existem diferentes tipos de evento - verifique `eventType` (`OutboundSyncEventType`) e `type` (`OutboundSyncType`).

Um caso de uso comum para esta API é implementar monitoramento personalizado. Você pode querer chamar periodicamente o endpoint `/count`
para consultar a contagem pendente para filtros dados.

A estrutura do objeto `PendingWebhookEvent` é a seguinte:

[inline-code-attrs-start title = 'Estrutura de PendingWebhookEvent'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum OutboundSyncEventType {
    Create: 0,
    Delete: 1,
    Update: 2
}

enum OutboundSyncType {
    /** Tarefa de sincronização específica do WordPress. **/
    WP: 0,
    Webhook: 1
}

interface PendingWebhookEvent {
    id: string
    /** O id do comentário associado ao evento. **/
    commentId: string
    /** O objeto de comentário para o evento no momento do evento. Começamos a adicioná-los em nov de 2023. **/
    comment: Comment
    /** Um id externo que pode estar associado ao comentário. **/
    externalId: string | null
    createdAt: Date
    tenantId: string
    attemptCount: number
    /** Definido antes da primeira tentativa e após cada falha. **/
    nextAttemptAt: Date
    /** Se este é um evento de criação, exclusão ou atualização... **/
    eventType: OutboundSyncEventType
    /** O tipo de sincronização a ser executada (WordPress, chamada de API, etc). **/
    type: OutboundSyncType
    /** O domínio que correspondeu ao comentário. Usamos este domínio para escolher a chave de API. **/
    domain: string
    /** O último erro ocorrido. Este tipo não é tipado e é um "dump" do que aconteceu. Geralmente contém um objeto com statusCode, body, e um mapa de headers. **/
    lastError: object | null
}
[inline-code-end]