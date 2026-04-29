FastComments disponibiliza quatro templates iniciais para que você não precise escrever um agente funcional do zero. Eles ficam acessíveis na [AI Agents page](https://fastcomments.com/auth/my-account/ai-agents) clicando em **Browse templates**.

Quando você escolhe um template:

1. O agente é criado com **Status: Execução de teste** e um nome interno baseado no template (`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`). Se esse nome já estiver em uso no seu tenant, é adicionado um sufixo numérico.
2. Você cai diretamente no formulário de edição com tudo pré-preenchido - prompt, gatilhos, ações permitidas e quaisquer limites. Um banner no topo diz "Criado a partir do template {templateName}. Revise as configurações abaixo, depois altere o status para Ativado quando estiver pronto."
3. Nada está habilitado ainda. O agente não agirá até você salvar e manter o modo de execução de teste (para observar) ou alterar para Ativado.

### The four templates

- **[Moderator](#template-moderator)** - revisa comentários novos e sinalizados, avisa infratores de primeira viagem e só escala para banimento após um aviso. Dispara em novos comentários e em cruzamentos de limiar de sinalização (limiar padrão de sinalização: 3). Ferramentas permitidas: `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Welcome Greeter](#template-welcome-greeter)** - responde calorosamente a comentaristas de primeira viagem com uma saudação curta e pessoal. Dispara em new-user-first-comment. Ferramenta permitida: `write_comment`.

- **[Top Comment Pinner](#template-top-comment-pinner)** - fixa comentários substanciais de nível superior quando eles ultrapassam um limiar de votos (padrão: 10), removendo primeiro a fixação do comentário previamente fixado. Dispara em cruzamentos de limiar de votos. Ferramentas permitidas: `pin_comment`, `unpin_comment`.

- **[Thread Summarizer](#template-thread-summarizer)** - publica um resumo neutro em um único parágrafo em threads longas após um atraso, e então o fixa. Dispara em novos comentários com um adiamento de 30 minutos para que a thread se estabilize antes da sumarização. Ferramentas permitidas: `write_comment`, `pin_comment`, `unpin_comment`.

### Customizing a template

Templates são pontos de partida, não contratos. Espera-se que você:

- Ajuste o **Initial prompt** para combinar com a voz da sua comunidade.
- Adicione ou remova **Triggers** para encaixar com a frequência que o agente deve rodar.
- Adicione **Approvals** para qualquer ação sensível - recomendamos fortemente colocar `ban_user` sob aprovação para templates no estilo moderador.
- Adicione **Community guidelines** para que o agente aplique sua política escrita de forma consistente. Veja [Community Guidelines](#community-guidelines).
- Defina **Budgets** por agente apropriados à quantidade de gatilhos que você espera.

O template é apenas um veículo que pré-preenche padrões sensatos; uma vez salvo, o agente é seu.