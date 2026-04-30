O FastComments fornece cinco modelos iniciais para que você não precise criar um agente funcional do zero. Eles estão acessíveis a partir da [página de Agentes de IA](https://fastcomments.com/auth/my-account/ai-agents) clicando em **Browse templates**.

Ao escolher um modelo:

1. O agente é criado com **Status: Execução de teste** e um nome interno baseado no modelo (`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`, `gaslight_detector`). Se esse nome já estiver em uso no seu tenant, um sufixo numérico é adicionado.
2. Você é levado diretamente ao formulário de edição com tudo pré-preenchido - **Prompt inicial**, gatilhos, ações permitidas e quaisquer limiares. Um banner no topo diz "Created from the {templateName} template. Review the settings below, then change status to Enabled when you're ready."
3. Nada está habilitado ainda. O agente não atuará até que você salve e mantenha a execução de teste ativada (para observar) ou mude para Habilitado.

### The five templates

- **[Moderador](#template-moderator)** - revisa comentários novos e sinalizados, avisa usuários na primeira infração e escala para ban apenas após um aviso. Dispara em novos comentários e quando o limiar de flags é atingido (limiar padrão de flags: 3). Ferramentas permitidas: `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Welcome Greeter](#template-welcome-greeter)** - responde calorosamente a comentaristas de primeira viagem com um breve e pessoal acolhimento. Dispara em new-user-first-comment. Ferramenta permitida: `write_comment`.

- **[Top Comment Pinner](#template-top-comment-pinner)** - fixa comentários de nível superior substanciais assim que ultrapassam um limite de votos (padrão: 10), desfixando primeiro o comentário préviamente fixado. Dispara quando o limiar de votos é ultrapassado. Ferramentas permitidas: `pin_comment`, `unpin_comment`.

- **[Thread Summarizer](#template-thread-summarizer)** - publica um resumo neutro de um único parágrafo em threads longas após um atraso, e então o fixa. Dispara em novos comentários com um adiamento de 30 minutos para que a thread se estabilize antes de resumir. Ferramentas permitidas: `write_comment`, `pin_comment`, `unpin_comment`.

- **[Gaslight Detector](#template-gaslight-detector)** - monitora edições de comentários para reescritas no meio da thread que distorcem respostas, restaura o texto original e envia uma DM ao autor. Dispara em edições de comentário. Ferramentas permitidas: `edit_comment`, `warn_user`, `send_dm`.

### Personalizando um modelo

Os modelos são pontos de partida, não contratos. Espera-se que você:

- Ajuste o **Prompt inicial** para corresponder à voz da sua comunidade.
- Adicione ou remova **Gatilhos** para adequar a frequência com que o agente deve atuar.
- Adicione **Aprovações** para qualquer ação sensível - recomendamos fortemente colocar `ban_user` atrás de aprovação para modelos no estilo moderador.
- Adicione **Diretrizes da comunidade** para que o agente aplique sua política escrita de forma consistente. Veja [Diretrizes da comunidade](#community-guidelines).
- Defina **Orçamentos** por agente apropriados à quantidade de gatilhos que você espera.

O modelo é apenas um veículo que pré-preenche valores padrão sensatos; uma vez salvo, o agente é seu.