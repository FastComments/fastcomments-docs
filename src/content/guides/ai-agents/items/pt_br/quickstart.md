Este é o caminho de cinco minutos de "temos AI Agents" para "um agente está respondendo a tráfego real, controlado por aprovações". Se preferir a versão completa, cada etapa contém um link para a página que a cobre em profundidade.

### 1. Abra a página de AI Agents

Acesse [AI Agents](https://fastcomments.com/auth/my-account/ai-agents) na sua conta. Na primeira vez que você chegar aqui, verá uma das opções:

- Um estado em branco com os botões **Browse templates** e **Start from scratch** (você tem agentes disponíveis para criar), ou
- Uma página de upsell se o seu plano não incluir agentes - veja [Plans and Eligibility](#plans-and-eligibility).

### 2. Escolha um template inicial

Clique em **Browse templates**. Escolha um dos:

- [Moderator](#template-moderator) - revisa comentários sinalizados ou novos, avisa os iniciantes e só escala para banimento após um aviso.
- [Welcome Greeter](#template-welcome-greeter) - responde a comentaristas de primeira viagem.
- [Top Comment Pinner](#template-top-comment-pinner) - fixa comentários relevantes quando eles ultrapassam um limite de votos.
- [Thread Summarizer](#template-thread-summarizer) - publica um resumo neutro em threads longas.

Cada template abre um formulário de edição pré-preenchido com **Status: Dry Run** já selecionado.

### 3. Revise e salve

No formulário de edição, faça no mínimo:

- **Internal name.** Um identificador curto usado nos painéis administrativos.
- **Display name.** O que aparece publicamente quando o agente posta um comentário.
- **Initial prompt.** Edite o prompt do template para corresponder ao seu tom de voz e às suas regras específicas.
- **Approvals.** Marque as ações que devem exigir revisão humana antes de entrar em vigor. Recomendamos pelo menos `ban_user` para qualquer agente de moderação. Veja [Approval Workflow](#approval-workflow).

Clique em **Save agent**.

### 4. Acompanhe em dry-run

O agente agora está ativo em **Dry Run**. Ele receberá seus gatilhos, chamará o modelo e registrará ações na página [Run History](#run-history) - com o selo **Dry Run** em cada linha - mas não executará ações reais. Visite alguns detalhes de execução (veja [Run Detail View](#run-detail-view)) e observe:

- As ações que o agente escolheu.
- A justificativa e a confiança de cada ação.
- O transcript completo do LLM.

Se o agente estiver tomando decisões com as quais você discorda, edite o prompt inicial ou marque mais aprovações.

### 5. Execute um teste contra comentários antigos

Na página de lista de agentes, clique em **Test run** na linha do agente. O formulário tem um único campo numérico **Days** (1 a 90). O tamanho da amostra e o limite máximo de comentários avaliados são exibidos apenas informativamente - eles são calculados no servidor, não definidos pelo usuário. O replay roda contra comentários históricos sem executar ações reais e informa o que o agente **teria** feito versus o que realmente aconteceu (o comentário foi aprovado depois, marcado como spam, excluído, e assim por diante). Veja [Test Runs (Replays)](#test-runs-replays).

### 6. Ative como Enabled

Quando estiver satisfeito com o dry-run e o resultado do replay, edite o agente e altere o **Status** para **Enabled**. A partir daí, ações reais passam a acontecer. A página Run History agora mostra execuções reais sem o selo de dry-run, e qualquer ação que você marcou para aprovação aparecerá na [caixa de entrada de aprovações](#approval-workflow).

### Próximos passos

- Configure [Budgets](#budgets-overview) e [Budget Alerts](#budget-alerts).
- Configure [Webhooks](#webhooks-overview) se quiser que sistemas externos reajam a eventos dos agentes.
- Adicione [Community Guidelines](#community-guidelines) para manter as decisões dos agentes alinhadas com a sua política escrita.