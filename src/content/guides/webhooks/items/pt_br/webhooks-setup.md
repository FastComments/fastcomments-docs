---
Siga os mesmos passos para `localhost` como faria em produção. Certifique‑se de que os domínios de produção e as chaves secretas da API estejam configurados.

Primeiro, navegue até o [Webhooks admin](https://fastcomments.com/auth/my-account/manage-data/webhooks). Isso está acessível via Gerenciar Dados -> Webhooks.

A página lista todos os webhooks da sua conta:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; alt='Página de administração de Webhooks listando cada webhook com sua URL, evento, domínio, método, status e contagem de eventos enfileirados'; title='Lista de Webhooks'; cacheBuster = 'v4' app-screenshot-end]

Clique em **Novo Webhook** para adicionar um. Cada webhook tem uma URL, um evento de comentário (criado, atualizado ou excluído), um domínio e um método HTTP:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks/new'; selector = '.content'; alt='Formulário de novo webhook com campos de URL, evento, domínio e método HTTP, além de Enviar Payload de Teste'; title='Novo Webhook'; cacheBuster = 'v4' app-screenshot-end]

Cada webhook é entregue de forma independente. Você pode enviar o mesmo evento para vários endpoints, e um webhook com escopo para **Todos os Domínios** recebe comentários de todos os domínios mesmo quando um webhook específico de domínio existe para o mesmo evento. A mesma URL, evento e domínio não podem ser adicionados duas vezes.

Antes de salvar, clique em **Enviar Payload de Teste** para verificar se o endpoint aceita uma solicitação assinada. Veja a próxima seção, "Teste", para detalhes.

Na lista, você pode editar, desativar, reativar ou excluir um webhook. Desativar mantém os eventos enfileirados até que o webhook seja reativado; excluir descarta‑os.

Webhooks também podem ser criados através da API, por exemplo pelo Zapier. Eles aparecem na mesma lista com a origem **API**. Veja Gerenciando Webhooks via API.

---