Ένας agent έχει μία από τρεις καταστάσεις:

### Disabled

The agent is turned off. No triggers are processed and the agent does not appear in the dispatch path. Its run history, analytics, and memory remain - if you re-enable it later, the historical data is still there.

Use `Disabled` when:
- Θέλετε να βγάλετε έναν agent εκτός ροτέισον χωρίς να τον χάσετε.
- Ένας agent συμπεριφέρεται λανθασμένα και χρειάζεται να τον σταματήσετε άμεσα ενώ διερευνάτε.
- Κάνετε εποχιακή εναλλαγή agents μέσα και έξω (π.χ. ένας υποδοχέας μόνο για τις γιορτές).

### Dry Run - default for new agents

The agent runs end-to-end - it processes triggers, calls the LLM, picks tool calls, computes justifications and confidence - but **no real action is taken**. Each run is recorded with the **Dry Run** badge in [Run History](#run-history).

Use `Dry Run` when:
- Ένας νέος agent μόλις βγήκε από το κουτί. Κάθε starter template μπαίνει σε dry-run.
- Έχετε επεξεργαστεί το prompt ή αλλάξει το σύνολο των triggers και θέλετε να δείτε πώς θα λειτουργήσει η αλλαγή πριν δεσμευτείτε.
- Τρέχετε μια [δοκιμαστική εκτέλεση / επανεκτέλεση](#test-runs-replays) (οι επανεκτελέσεις αναγκάζουν dry-run ανεξαρτήτως της κατάστασης του agent).

The platform charges tokens for dry-run runs - the LLM call still happens, only the side-effects are skipped. Budget caps apply to dry-run too. See [Επισκόπηση Προϋπολογισμών](#budgets-overview).

### Enabled

The agent takes real actions. Tool calls execute - or get queued for [έγκριση](#approval-workflow) if the action is gated.

Use `Enabled` after dry-run output looks correct.

### Switching status

You can flip between any two statuses on the edit form. Switching from Dry Run to Enabled does not retroactively re-execute the dry-run actions - those stay as dry-run history. New triggers from that moment forward run live.

Switching from Enabled to Disabled mid-run does **not** abort an in-flight run. The currently-executing trigger finishes (with whatever it has already started); the next trigger is dropped because the agent is now Disabled.

### Status during billing problems

If your tenant's billing becomes invalid, all agents are effectively paused regardless of saved status - triggers are dropped with `BILLING_INVALID` until billing is restored. The saved status field is not changed; the dispatcher just refuses to run. See [Σχέδια και Επιλεξιμότητα](#plans-and-eligibility).