**ID do Template:** `tos_enforcer`

O template Moderador é o ponto de partida recomendado se seu objetivo é reduzir a carga de moderação manual. Ele revisa comentários novos e sinalizados e aplica as regras da sua comunidade.

### Prompt inicial incorporado

[inline-code-attrs-start title = 'Prompt Inicial do Modelo Moderador'; type='text' inline-code-attrs-end]
[inline-code-start]
You are a terms-of-service enforcement agent. Review each new comment against the community guidelines. Mark clear spam or policy violations. Issue warnings for first-offense borderline content. Escalate ban decisions only for repeat or severe violations. If a comment is clearly within the rules, approve it so it becomes visible (relevant for pre-moderation tenants). Stay out of political or subjective debates, focus on the rules as written.
[inline-code-end]

Você quase sempre vai querer **aumentar este prompt** com exemplos concretos do que seu site permite e não permite. A própria política de escalonamento da plataforma (advertir antes de banir, pesquisar na memória antes de banir) já está incorporada no prompt do sistema que o agente recebe, então você não precisa repeti-la.

### Gatilhos

- **Novo comentário publicado** (`COMMENT_ADD`) - o agente analisa cada comentário novo.
- **Comentário ultrapassa um limiar de sinalização** (`COMMENT_FLAG_THRESHOLD`, limiar padrão: 3) - o agente reavalia um comentário que outros usuários sinalizaram.

### Ferramentas permitidas

- [`mark_comment_approved`](#tools-overview) - útil para instâncias de pré-moderação, onde o agente libera comentários limpos e esconde o resto.
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

Ele não pode publicar comentários, votar, fixar, bloquear, conceder insígnias ou enviar e-mail — o prompt é intencionalmente restrito.

### Adições recomendadas antes de entrar em produção

- **Defina as [Diretrizes da Comunidade](#community-guidelines).** Algumas frases de política escrita são suficientes; o agente as aplica em cada execução.
- **Coloque `ban_user` sob [aprovação](#approval-workflow).** Isso é ativado por padrão na região da UE (veja [Conformidade com o Artigo 17 da DSA da UE](#eu-dsa-compliance)) e é recomendado em todos os lugares.
- **Considere também colocar `mark_comment_spam` sob aprovação** se você tiver conteúdo de baixo volume, mas de alto risco.
- **Coloque `mark_comment_approved` sob aprovação se você usar pré-moderação.** Aprovar um comentário ruim o coloca diante dos leitores; mantenha sob aprovação até que o agente tenha conquistado confiança através do modo dry-run.
- **Marque "Incluir fator de confiança do autor do comentário, idade da conta, histórico de banimentos e comentários recentes"** em [Opções de Contexto](#context-options). O modelo vai advertir muito menos agressivamente quando puder ver que alguém é um usuário de boa-fé de longa data.

### Período de dry-run recomendado

Execute este template em [dry-run](#dry-run-mode) por pelo menos uma semana com seu tráfego real antes de ativá-lo. Use [Execuções de Teste (Replays)](#test-runs-replays) para também pré-visualizar contra os últimos 30 dias.

---