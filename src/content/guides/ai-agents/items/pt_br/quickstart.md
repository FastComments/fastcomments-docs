Este é o caminho de cinco minutos de "temos Agentes de IA" até "um agente está respondendo ao tráfego ao vivo, sujeito a aprovações." Se quiser a versão detalhada, cada etapa tem um link para a página que a aborda em profundidade.

### 1. Open the AI Agents page

Go to [Agentes de IA](https://fastcomments.com/auth/my-account/ai-agents) in your account. The first time you land here you will see either:

- A blank-state with a **Explorar modelos** and **Começar do zero** button (you have agents available to create), or
- An upsell page if your plan does not include agents - see [Planos e Elegibilidade](#plans-and-eligibility).

### 2. Pick a starter template

Click **Explorar modelos**. Pick one of:

- [Moderador](#template-moderator) - revisa comentários sinalizados ou novos, avisa usuários de primeira vez, escala para banir somente após um aviso.
- [Agente de Boas-vindas](#template-welcome-greeter) - responde a comentaristas de primeira vez.
- [Fixador de Comentários Principais](#template-top-comment-pinner) - fixa comentários substanciais quando ultrapassam um limite de votos.
- [Resumidor de Discussões](#template-thread-summarizer) - publica um resumo neutro em threads longas.

Each template lands on a pre-filled edit form with **Status: Simulação** already selected.

### 3. Review and save

On the edit form, do at minimum:

- **Nome interno.** Um identificador curto usado nos painéis de administração.
- **Nome de exibição.** O que aparece publicamente quando o agente publica um comentário.
- **Prompt inicial.** Edite o prompt do modelo para combinar com sua voz e suas regras específicas.
- **Aprovações.** Marque as ações que devem exigir revisão humana antes de entrarem em vigor. Recomendamos pelo menos `ban_user` para qualquer agente de moderação. Veja [Fluxo de Aprovação](#approval-workflow).

Click **Salvar agente**.

### 4. Watch it in dry-run

O agente está agora ativo em **Simulação**. Ele receberá seus gatilhos, chamará o modelo e registrará ações na página [Histórico de Execuções](#run-history) - com o selo **Simulação** em cada linha - mas não executa ações reais. Visite alguns detalhes das execuções (veja [Visualização de Detalhe da Execução](#run-detail-view)) e verifique:

- As ações que o agente escolheu.
- A justificativa e o nível de confiança para cada ação.
- A transcrição completa do LLM.

Se o agente estiver tomando decisões com as quais você discorda, edite o prompt inicial ou marque mais aprovações.

### 5. Run a test against past comments

From the agents list page, click **Executar teste** on the agent's row. The form has a single **Dias** numeric input (1 to 90). Sample size and the hard cap on comments evaluated are shown informationally - they are computed server-side, not user-set. The replay runs against historical comments without taking real actions and reports what the agent **would** have done versus what actually happened (was the comment later approved, marked spam, deleted, and so on). See [Execuções de Teste (Reproduções)](#test-runs-replays).

### 6. Flip to Enabled

When you are happy with the dry-run and replay output, edit the agent and change **Status** to **Habilitado**. From here on, real actions land. The Run History page now shows live runs without the dry-run badge, and any action you marked for approval appears in the [caixa de entrada de aprovações](#approval-workflow).

### What's next

- Set [Orçamentos](#budgets-overview) and [Alertas de Orçamento](#budget-alerts).
- Configure [Webhooks](#webhooks-overview) if you want external systems to react to agent events.
- Add [Diretrizes da Comunidade](#community-guidelines) to keep agent decisions aligned with your written policy.