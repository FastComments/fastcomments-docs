**Template ID:** `tos_enforcer`

O template Moderator é o ponto de partida recomendado se seu objetivo for reduzir a carga de moderação manual. Ele analisa comentários novos e sinalizados e aplica as regras da sua comunidade.

Você quase sempre vai querer **complementar o prompt integrado** com exemplos concretos do que seu site permite e não permite. A própria política de escalonamento da plataforma (advertir antes de banir, pesquisar a memória antes de banir) já está incorporada ao prompt do sistema que o agente recebe, então você não precisa repeti-la.

### Gatilhos

- **New comment posted** (`COMMENT_ADD`) - o agente analisa todo comentário novo.
- **Comment crosses a flag threshold** (`COMMENT_FLAG_THRESHOLD`, default threshold: 3) - o agente reavalia um comentário que outros usuários sinalizaram.

### Ferramentas permitidas

- [`mark_comment_approved`](#tools-overview) - útil para locatários com pré-moderação, onde o agente libera comentários limpos e oculta o restante.
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

Ele não pode postar comentários, votar, fixar, bloquear, conceder distintivos ou enviar e-mail — o prompt é intencionalmente restrito.

### Adições recomendadas antes de ir ao vivo

- **Defina as [Diretrizes da Comunidade](#community-guidelines).** Algumas frases de política escrita são suficientes; o agente as aplica a cada execução.
- **Restringir `ban_user` por meio de [aprovação](#approval-workflow).** Isso vem ativado por padrão na região da UE (veja [Conformidade com o Artigo 17 da DSA da UE](#eu-dsa-compliance)) e é recomendado em todos os lugares.
- **Considere também restringir `mark_comment_spam` por aprovação** se você tiver conteúdo de baixo volume, mas de alto risco.
- **Restringir `mark_comment_approved` por aprovação se você usar pré-moderação.** Aprovar um comentário ruim o coloca diante dos leitores; restrinja essa ação até que o agente ganhe confiança através do modo dry-run.
- **Marque "Incluir fator de confiança do comentarista, idade da conta, histórico de banimentos e comentários recentes"** em [Opções de Contexto](#context-options). O modelo advertirá com muito menos agressividade quando puder ver que alguém é um usuário de boa-fé há muito tempo.

### Período de dry-run recomendado

Execute este template no [modo dry-run](#dry-run-mode) por pelo menos uma semana contra seu tráfego real antes de ativá-lo. Use [Execuções de Teste (Replays)](#test-runs-replays) para também pré-visualizar em relação aos 30 dias anteriores.