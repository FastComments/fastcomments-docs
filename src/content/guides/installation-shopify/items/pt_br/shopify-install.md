### Instalar pela Shopify App Store

1. Abra a [listagem do FastComments na Shopify App Store](https://apps.shopify.com/fastcomments).
2. Clique em **Adicionar app** e escolha o plano que você deseja durante o fluxo de instalação.
3. O Shopify redireciona você de volta para o admin do FastComments dentro do Shopify quando a instalação for concluída.

Isso conclui a instalação. Não há nada para colar nos arquivos do seu tema.

### O que é configurado para você

A instalação executa tudo o que você faria manualmente:

- Um tenant do FastComments é criado para sua loja e vinculado ao domínio da sua loja.
- A URL da sua loja é adicionada aos domínios autorizados do tenant, para que os comentários carreguem sem erro de domínio.
- Um metafield de loja `fastcomments.tenant_id` é escrito para que cada bloco saiba contra qual tenant renderizar.
- O login único (single sign-on) para seus clientes da Shopify é ativado por padrão.
- A cobrança é feita pelo Shopify Managed Pricing. As cobranças aparecem na sua fatura regular do Shopify. Faça upgrade, downgrade ou cancele em **Settings > Apps and sales channels > FastComments** no admin do Shopify.

Se sua loja já era cliente do FastComments antes de você instalar o app, a instalação reutiliza o tenant existente em vez de criar um novo.

### O painel administrativo incorporado

Ao abrir o app FastComments a partir do seu admin do Shopify, você chega a um painel com blocos de um clique que levam ao backend completo do FastComments:

- **Painel**: configurações da conta, uso e detalhes da assinatura.
- **Fila de moderação**: aprovar, rejeitar e responder comentários em toda a sua loja.
- **Personalizar**: ajustar cores do widget, fontes, regras de moderação e configuração.
- **Ratings & Reviews Helper**: configurar avaliações por estrelas e perguntas de review se desejar usar o bloco Reviews Summary.

Cada bloco abre o FastComments com um link de login de uso único, portanto você não precisa de um login separado.

### Próximo: adicione blocos à sua loja

Abra o editor de temas do Shopify (**Online Store > Themes > Customize**), abra o template ao qual você quer adicionar comentários ou avaliações, e clique em **Adicionar bloco**. Os blocos do FastComments aparecem em **Apps**. O restante deste guia aborda cada um deles.