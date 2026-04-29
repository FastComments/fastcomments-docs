Un agente ha uno dei tre stati:

### Disabled

L'agente è spento. Nessun trigger viene elaborato e l'agente non appare nel dispatch path. La sua run history, le analytics e la memoria rimangono - se lo riattivi in seguito, i dati storici sono ancora presenti.

Use `Disabled` when:
- Vuoi rimuovere un agente dalla rotazione senza perderlo.
- Un agente si comporta in modo anomalo e hai bisogno di fermarlo immediatamente mentre indaghi.
- Stai ruotando stagionalmente gli agenti dentro e fuori (es. un greeter attivo solo per le festività).

### Dry Run - default for new agents

The agent runs end-to-end - it processes triggers, calls the LLM, picks tool calls, computes justifications and confidence - but **no real action is taken**. Each run is recorded with the **Dry Run** badge in [Run History](#run-history).

Use `Dry Run` when:
- Un agente nuovo è appena fuori dalla scatola. Every starter template lands in dry-run.
- Hai modificato il prompt o il set di trigger e vuoi vedere come si comporta la modifica prima di impegnarla.
- Stai eseguendo una [test run / replay](#test-runs-replays) (replays force dry-run regardless of agent status).

The platform charges tokens for dry-run runs - the LLM call still happens, only the side-effects are skipped. Budget caps apply to dry-run too. See [Budgets Overview](#budgets-overview).

### Enabled

L'agente compie azioni reali. Tool calls execute - or get queued for [approval](#approval-workflow) if the action is gated.

Use `Enabled` after dry-run output looks correct.

### Switching status

Puoi flip tra qualsiasi coppia di stati nel modulo di modifica. Switching from Dry Run to Enabled does not retroactively re-execute the dry-run actions - those stay as dry-run history. New triggers from that moment forward run live.

Switching from Enabled to Disabled mid-run does **not** abort an in-flight run. The currently-executing trigger finishes (with whatever it has already started); the next trigger is dropped because the agent is now Disabled.

### Status during billing problems

If your tenant's billing becomes invalid, all agents are effectively paused regardless of saved status - triggers are dropped with `BILLING_INVALID` until billing is restored. The saved status field is not changed; the dispatcher just refuses to run. See [Plans and Eligibility](#plans-and-eligibility).