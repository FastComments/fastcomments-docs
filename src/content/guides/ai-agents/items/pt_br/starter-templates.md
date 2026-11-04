O FastComments inclui cinco modelos iniciais para que você não precise escrever um agente funcional do zero. Eles podem ser acessados na [página de AI Agents](https://fastcomments.com/auth/my-account/ai-agents) clicando em **Browse templates**.

Ao escolher um modelo:

1. O agente é criado com **Status: Dry Run** e um nome interno baseado no modelo (`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`, `gaslight_detector`). Se esse nome já estiver em uso no seu tenant, um sufixo numérico é adicionado.
2. Você é levado diretamente ao formulário de edição com tudo pré-preenchido - prompt, gatilhos, ações permitidas e quaisquer limites. Um banner no topo exibe: "Created from the {templateName} template. Review the settings below, then change status to Enabled when you're ready."
3. Nada está habilitado ainda. O agente não agirá até que você salve e mantenha o dry-run ativado (para observar) ou altere para Enabled.

### Os cinco modelos

- **[Moderator](#template-moderator)** - revisa comentários novos e sinalizados, adverte infratores de primeira viagem e só escala para banimento após uma advertência. Aciona em novos comentários e ao cruzar o limite de sinalizações (limite padrão: 3). Ferramentas permitidas: `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Welcome Greeter](#template-welcome-greeter)** - responde calorosamente a comentaristas de primeira viagem com uma saudação curta e pessoal. Aciona na primeira publicação de novos usuários. Ferramenta permitida: `write_comment`.

- **[Top Comment Pinner](#template-top-comment-pinner)** - fixa comentários de nível superior substanciais quando eles cruzam um limite de votos (padrão: 10), desafixando primeiro o comentário fixado anteriormente. Aciona ao cruzar limites de votos. Ferramentas permitidas: `pin_comment`, `unpin_comment`.

- **[Thread Summarizer](#template-thread-summarizer)** - publica um resumo neutro de um único parágrafo em threads longas após um atraso e, em seguida, o fixa. Aciona em novos comentários com um adiamento de 30 minutos para que a thread se estabilize antes do resumo. Ferramentas permitidas: `write_comment`, `pin_comment`, `unpin_comment`.

- **[Gaslight Detector](#template-gaslight-detector)** - monitora edições de comentários em busca de reescritas no meio da thread que distorçam as respostas, restaura o texto original e envia uma DM ao autor. Aciona em edições de comentários. Ferramentas permitidas: `edit_comment`, `warn_user`, `send_dm`.

### Personalizando um modelo

Os modelos são pontos de partida, não contratos. Espera-se que você:

- Ajuste o **Initial prompt** para corresponder ao tom da sua comunidade.
- Adicione ou remova **Triggers** para adequar a frequência de execução do agente.
- Adicione **Approvals** para qualquer ação sensível - recomendamos fortemente exigir aprovação para `ban_user` em modelos do tipo moderador.
- Adicione **Community guidelines** para que o agente aplique sua política escrita de forma consistente. Veja [Community Guidelines](#community-guidelines).
- Defina **Budgets** por agente adequados à quantidade de gatilhos esperada.

O modelo é apenas um veículo que pré-preenche padrões sensatos; uma vez salvo, o agente é seu.