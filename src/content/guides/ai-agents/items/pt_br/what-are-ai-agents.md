Um **AI Agent** é um trabalhador autônomo, limitado ao seu tenant FastComments, que observa eventos em sua comunidade e age em seu nome.

Cada agente tem três coisas que você controla:

1. **Uma personalidade.** Um prompt inicial em texto livre que define tom, papel e estilo de tomada de decisão (“Você é um saudador caloroso da comunidade”, “Você aplica as regras da comunidade, mas tende a avisar antes de banir”, etc.).
2. **Um ou mais gatilhos.** Uma lista de eventos que despertam o agente – um novo comentário, um comentário que ultrapassa um limite de votos ou sinalizações, uma ação de moderador, o primeiro comentário de um usuário no site, entre outros. A lista completa está em [Trigger Events Overview](#triggers-overview).
3. **Uma lista de permissões de ferramentas.** O que o agente tem permissão para fazer – publicar um comentário, votar, fixar, bloquear, marcar como spam, banir um usuário, avisar via DM, conceder um distintivo, enviar e‑mail, salvar e pesquisar uma memória compartilhada. A lista completa está em [Allowed Tool Calls Overview](#tools-overview).

Quando um gatilho dispara, o agente recebe uma mensagem de contexto descrevendo o que aconteceu (o comentário, a página, contexto opcional de thread/usuário/página) e é alimentado com seu prompt inicial e as diretrizes da sua comunidade. Em seguida, ele chama ferramentas para agir, registrando uma justificativa e uma pontuação de confiança a cada chamada.

### Agentes executam de forma assíncrona

Os agentes **nunca bloqueiam a ação do usuário que os disparou**. Um leitor publica um comentário, o comentário é salvo e exibido na thread, a resposta é retornada e só *então* o agente o processa – imediatamente ou após um atraso configurado (veja [Deferred Triggers](#trigger-deferred-delay)). Nada que o agente faz adiciona latência à experiência do usuário.

### Por que usá-los

- **Moderar em escala.** Marcar spam óbvio e banir infratores recorrentes sem precisar monitorar a fila 24 horas por dia.  
- **Dar boas‑vindas a novos comentaristas.** Responder aos comentaristas de primeira vez com a sua voz.  
- **Destacar o melhor conteúdo.** Fixar comentários de nível superior substanciais quando ultrapassam um limite de votos.  
- **Aplicar suas diretrizes de forma consistente.** Aplicar o mesmo texto de política a cada comentário borderline.  
- **Resumir threads longas.** Publicar resumos neutros de debates de várias páginas.

### O que mantém você no controle

- **Modo de teste (dry‑run).** Cada novo agente é entregue em **Dry Run**: ele processa gatilhos, executa o modelo e registra o que ele *faria*, mas não realiza ações reais. Veja [Dry-Run Mode](#dry-run-mode).  
- **Aprovações.** Qualquer subconjunto de ações pode ser condicionado à aprovação humana. Veja [Approval Workflow](#approval-workflow).  
- **Orçamentos por agente e por conta.** Limites diários e mensais rígidos. Veja [Budgets Overview](#budgets-overview).  
- **Lista de permissões de ferramentas.** Ferramentas não permitidas são removidas da paleta do modelo – o agente literalmente não pode solicitá‑las. Veja [Allowed Tool Calls Overview](#tools-overview).  
- **Campos de auditoria em cada ação.** O modelo deve incluir uma justificativa e uma pontuação de confiança. Ambos aparecem na linha do tempo da execução e em cada aprovação. Veja [Run Detail View](#run-detail-view).  
- **Artigo 17 do EU DSA.** Na região da UE, banimentos totalmente automatizados são bloqueados. Veja [EU DSA Article 17 Compliance](#eu-dsa-compliance).  
- **Sem treinamento nos seus dados.** FastComments usa provedores que não treinam com seus prompts ou comentários.

### Onde eles se encaixam ao lado da moderação humana

Agentes e moderadores humanos compartilham a mesma plataforma de comentários: os agentes realizam ações pelos mesmos canais (aprovar, spam, banir, distintivo, fixar, bloquear, escrever) e essas ações aparecem nos mesmos [Comment Logs](/guide-moderation.html#comment-logs), na mesma [Moderation Page](/guide-moderation.html#moderate-comments-page) e nos mesmos fluxos de notificação. Agentes e humanos veem o trabalho um do outro e podem reagir mutuamente – as ações de moderador são, por si só, gatilhos válidos para agentes (veja [Trigger: Moderator Reviewed Comment](#trigger-moderator-reviewed) e afins).

Se você também precisar de pessoas na fila, a FastComments oferece [Human Moderation Services](/guide-moderation.html#human-moderation-services): nossos moderadores trabalham ao lado dos seus agentes, no seu painel, sob suas diretrizes.