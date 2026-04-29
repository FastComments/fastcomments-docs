Um **AI Agent** é um trabalhador autônomo, limitado ao seu FastComments tenant, que vigia eventos na sua comunidade e toma ações em seu nome.

Cada agente tem três coisas que você controla:

1. **Uma personalidade.** Um prompt inicial em texto livre que define tom, papel e estilo de tomada de decisão ("Você é um recepcionista caloroso da comunidade", "Você aplica as regras da comunidade mas tende a advertir em vez de banir", e assim por diante).
2. **Um ou mais gatilhos.** Uma lista de eventos que despertam o agente - um novo comentário, um comentário ultrapassando um limite de votos ou sinalizações, uma ação de moderador, o primeiro comentário de um usuário no site, entre outros. A lista completa está em [Visão Geral de Eventos de Gatilho](#triggers-overview).
3. **Uma lista de permissões de ferramentas.** O que o agente tem permissão para fazer - publicar um comentário, votar, fixar, bloquear, marcar como spam, banir um usuário, avisar por DM, conceder um distintivo, enviar e-mail, salvar e pesquisar uma memória compartilhada. A lista completa está em [Visão Geral de Chamadas de Ferramentas Permitidas](#tools-overview).

Quando um gatilho é acionado, o agente recebe uma mensagem de contexto descrevendo o que aconteceu (o comentário, a página, contexto opcional de thread/usuário/página) e é apresentado ao seu prompt inicial e às diretrizes da sua comunidade. Em seguida ele chama ferramentas para agir, registrando uma justificativa e uma pontuação de confiança em cada chamada.

### Agentes rodam de forma assíncrona

Os agentes **nunca bloqueiam a ação do usuário que os acionou**. Um leitor publica um comentário, o comentário é salvo e mostrado na thread, a resposta é retornada, e só *então* o agente executa sobre ele - seja imediatamente ou após um atraso configurado (veja [Gatilhos Retardados](#trigger-deferred-delay)). Nada do que o agente faz adiciona latência à experiência voltada ao usuário.

### Por que usá-los

- **Moderar em escala.** Marcar spam óbvio e banir reincidentes sem vigiar a fila o tempo todo.
- **Dar boas-vindas a novos comentaristas.** Responder aos comentaristas de primeira vez com sua voz.
- **Destacar o melhor conteúdo.** Fixar comentários de nível superior substanciais assim que ultrapassarem um limite de votos.
- **Aplicar suas diretrizes de forma consistente.** Aplicar o mesmo texto de política a todo comentário borderline.
- **Resumir longas discussões.** Publicar resumos neutros de debates de várias páginas.

### O que mantém você no controle

- **Modo Dry Run.** Todo agente novo é entregue em **Modo Dry Run**: ele processa gatilhos, executa o modelo e registra o que *faria*, mas não toma ações reais. Veja [Modo Dry Run](#dry-run-mode).
- **Aprovações.** Qualquer subconjunto de ações pode ficar sujeito à aprovação humana. Veja [Fluxo de Aprovação](#approval-workflow).
- **Orçamentos por agente e por conta.** Limites rígidos diários e mensais. Veja [Visão Geral de Orçamentos](#budgets-overview).
- **Lista de ferramentas permitidas.** Ferramentas não permitidas são removidas da paleta do modelo - o agente literalmente não pode solicitá-las. Veja [Visão Geral de Chamadas de Ferramentas Permitidas](#tools-overview).
- **Campos de auditoria em cada ação.** O modelo deve incluir uma justificativa e uma pontuação de confiança. Ambos aparecem na linha do tempo da execução e em cada aprovação. Veja [Visão Detalhada da Execução](#run-detail-view).
- **Artigo 17 do DSA da UE.** Na região da UE, banimentos totalmente automatizados são bloqueados. Veja [Conformidade com o Artigo 17 do DSA da UE](#eu-dsa-compliance).
- **Nenhum treinamento com seus dados.** FastComments usa provedores que não treinam com seus prompts ou comentários.

### Onde eles se encaixam ao lado da moderação humana

Agentes e moderadores humanos compartilham a mesma plataforma de comentários: agentes tomam ações pelos mesmos canais (aprovar, marcar como spam, banir, conceder distintivo, fixar, bloquear, escrever) e essas ações aparecem nos mesmos [Registros de Comentários](/guide-moderation.html#comment-logs), na mesma [Página de Moderação](/guide-moderation.html#moderate-comments-page) e nos mesmos fluxos de notificação. Agentes e humanos veem o trabalho uns dos outros e podem reagir entre si - ações de moderador são por si mesmas gatilhos válidos para agentes (veja [Gatilho: Comentário Revisado pelo Moderador](#trigger-moderator-reviewed) e afins).

---