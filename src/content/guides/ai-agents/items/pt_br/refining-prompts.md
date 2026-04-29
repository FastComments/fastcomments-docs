**Refine Prompt** é o fluxo de trabalho para editar o [prompt inicial](#personality-prompt) de um agente em resposta a decisões específicas com as quais você discorda. Ele é iniciado a partir da [caixa de entrada de aprovações](#approval-workflow).

### When to use it

Quando você se pega rejeitando o mesmo tipo de aprovação repetidamente — "o agente continua querendo banir pessoas por usar linguagem forte sem um alvo" — o prompt do agente é a alavanca para corrigir isso. Refine Prompt é uma maneira guiada de:

1. Escolher uma aprovação específica que represente a decisão ruim.
2. Editar o prompt com todo o contexto do que o agente fez e por quê.
3. Salvar o novo prompt no agente.

O resultado é um agente que, daqui para frente, provavelmente não tomaria a mesma decisão.

### Launching the flow

From the approvals inbox at `/auth/my-account/ai-agent-approvals`:

1. Open a **rejected** approval. The route hard-rejects anything except REJECTED - pending and execution-failed approvals are not eligible.
2. Click **Refine prompt**.

You land on the prompt-refine UI at `/auth/my-account/ai-agent-approvals/:approvalId/refine-prompt`.

### What the page shows

- **The approval** - the agent's `toolName` and `justification` for the rejected decision (the full LLM transcript is not shown here).
- **The current prompt** - the agent's saved [initial prompt](#personality-prompt).
- **A feedback input** - you type **feedback** describing what should change (up to 2000 characters). The LLM then generates the proposed new prompt from your feedback.
- **Unified inline diff** - a single inline diff between the current and the proposed prompt (red for removed, green for added).

The approval context stays pinned at the top so you can keep referring to "the case I'm fixing for" while editing.

### Save

Saving updates the agent's `initialPrompt` field. Past runs (and past approvals) are not retroactively rerun - the new prompt only affects future triggers. If you want to verify the new prompt fixes the problem, run a [test run / replay](#test-runs-replays) against the last 7 days and look for whether the new prompt would still produce the rejected approval.

### What the flow does not do

- It does not edit the **community guidelines** - that field has its own editor on the main agent edit form.
- It does not edit **triggers**, **allowed tools**, or **approval gating** - those remain on the main edit form.
- It does not version the prompt with rollback. The previous prompt is not stored in a separate history collection. If you need to roll back, copy the current prompt into your own tracking system before editing.

### Why pair refine with replay

Editing a prompt without testing the result is faith-based. The recommended cycle:

1. Reject an approval.
2. Refine the prompt.
3. Run a [test run](#test-runs-replays) against the last 7 days.
4. Look at the **Deltas** tab. Did the new prompt move the bad decision out of "would do" and into "would not do"? Did it accidentally move good decisions out too?
5. Iterate.

Three or four cycles of refine + replay is usually enough to get a stable prompt for a moderation agent.

### Direct edit alternative

You do not have to use Refine Prompt - you can also just edit the agent on the main edit form. Refine Prompt's only advantage is that it pins a specific failing case so you do not lose track of what you are fixing for.