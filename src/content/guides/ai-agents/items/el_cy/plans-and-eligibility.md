AI Agents are available on the **Flex** and **Pro** plans. The Creator plan does not include them.

### Όρια σε επίπεδο σχεδίου

Each plan tier sets:

- **Προεπιλεγμένα ημερήσια και μηνιαία όρια προϋπολογισμού.** You can lower these per-agent; raising the per-account cap requires a plan with a higher ceiling. See [Επισκόπηση Προϋπολογισμών](#budgets-overview).

The exact numbers are shown on the [σελίδα τιμολόγησης](https://fastcomments.com/traffic-pricing) and on your account's billing page. They are also shown inline on the agent edit form so you never have to leave the form to find your cap.

FastComments Pro includes $200/mo worth of AI usage. Flex is billed at the rate of $20 per million tokens for all models (currently either GLM 5.1 or gpt-oss-120B-turbo).

### Η χρέωση πρέπει να είναι έγκυρη

AI Agents only run when the tenant has **έγκυρα στοιχεία χρέωσης**. If the payment method becomes invalid, all agents are paused and the AI Agents page surfaces a banner directing whoever has the **Billing Admin** role to update billing. Agents resume on their own once billing is restored - no replay or backfill of triggers that fired during the outage.

This is a hard prerequisite: token spend bills against your account, so the platform will not dispatch any LLM call without a working payment method.

### Ποιος μπορεί να διαχειρίζεται agents

The agent admin pages are gated behind the **Customization Admin** dashboard role. **Comment Moderator Admins** can review and decide approvals (see [Διαδικασία Έγκρισης](#approval-workflow)) but cannot create or edit agents. **Billing Admins** receive [ειδοποιήσεις προϋπολογισμού μέσω email](#budget-alerts) regardless of whether they have agent access.