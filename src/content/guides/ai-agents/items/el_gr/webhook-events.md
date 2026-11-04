Υπάρχουν τέσσερις τύποι συμβάντων webhook agent. Κάθε συμβάν έχει μια αριθμητική τιμή enum (χρησιμοποιείται στα payloads) και ένα κανονικό όνομα συμβολοσειράς (χρησιμοποιείται στο πεδίο `event` του envelope και στην κεφαλίδα HTTP `X-FastComments-Agent-Event`).

| Όνομα συμβάντος | Enum | Ενεργοποιείται όταν |
|---|---|---|
| `trigger.succeeded` | 0 | Μια εκτέλεση agent ολοκληρώνεται με κατάσταση `SUCCESS`. |
| `trigger.failed` | 1 | Μια εκτέλεση agent ολοκληρώνεται με κατάσταση `ERROR`. |
| `approval.requested` | 2 | Μια έγκριση μπαίνει σε ουρά με κατάσταση `PENDING`. |
| `approval.decided` | 3 | Μια έγκριση μεταβαίνει σε `APPROVED`, `REJECTED` ή `EXECUTION_FAILED`. |

### `trigger.succeeded`

Ενεργοποιείται αφού η εκτέλεση του agent ολοκληρωθεί χωρίς σφάλμα. Το πεδίο `data` του payload περιλαμβάνει:

- `triggerId` - το μοναδικό αναγνωριστικό εκτέλεσης.
- `triggerType` - το [enum αιτίας ενεργοποίησης](#triggers-overview) που ξεκίνησε την εκτέλεση.
- `status` - `SUCCESS` (συμβολοσειρά).
- `tokensUsed` - tokens που καταναλώθηκαν σε αυτή την εκτέλεση.
- `wasDryRun` - true αν ο agent βρισκόταν σε [λειτουργία dry-run](#dry-run-mode).
- `actions` - πίνακας εγγραφών `TenantAgentAction` (βλ. [Webhook Payloads](#webhook-payloads)).
- `commentId`, `url`, `urlId` - αν η ενεργοποίηση τα είχε.

Αν η εκτέλεση δεν έκανε καμία ενέργεια, ο πίνακας `actions` είναι κενός - αυτή είναι μια επιτυχημένη εκτέλεση «ο agent αποφάσισε να μην κάνει τίποτα», κάτι χρήσιμο να γνωρίζετε.

### `trigger.failed`

Ενεργοποιείται όταν μια εκτέλεση παρουσιάσει σφάλμα. Ίδια δομή payload με το `trigger.succeeded`, με `status: 'ERROR'` και ένα επιπλέον πεδίο `errorMessage` που περιγράφει τι πήγε στραβά. Πιθανά σφάλματα περιλαμβάνουν αποτυχίες κλήσεων LLM, αποτυχίες dispatch εργαλείων και εξάντληση budget μέσα στην εκτέλεση.

Ο `actions` μπορεί ακόμα να περιέχει εγγραφές για κλήσεις εργαλείων που ολοκληρώθηκαν πριν το σφάλμα.

### `approval.requested`

Ενεργοποιείται τη στιγμή που μια έγκριση μπαίνει σε ουρά με κατάσταση `PENDING`. Το payload περιλαμβάνει:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status: 'PENDING'`.
- `args` - τα ορίσματα του εργαλείου **περασμένα αυτούσια** από την κλήση LLM. Η δομή είναι ανά εργαλείο και δεν αποτελεί σταθερό δημόσιο συμβόλαιο - το schema μπορεί να αλλάξει καθώς προστίθενται νέα εργαλεία.
- `createdAt`.
- `justification`, `confidence` - αν ο agent τα παρείχε.
- `contextSnapshot` - το πλαίσιο σχολίου / σελίδας στο οποίο σχετίζεται η έγκριση.

Χρήσιμο για προώθηση εκκρεμών εγκρίσεων σε κανάλι chat ops: ένα Slack bot εγγεγραμμένο στο `approval.requested` μπορεί να δημοσιεύσει την ενέργεια και την αιτιολόγηση σε κανάλι συντονισμού για γρήγορη επισκόπηση.

### `approval.decided`

Ενεργοποιείται όταν μια έγκριση βγαίνει από την κατάσταση `PENDING`. Το payload περιλαμβάνει:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status` - `APPROVED`, `REJECTED` ή `EXECUTION_FAILED`.
- `decidedBy` - το αναγνωριστικό χρήστη του συντονιστή που αποφάσισε.
- `decidedAt` - πότε αποφάσισε.
- `executedAt` - αν APPROVED, πότε η πλατφόρμα εκτέλεσε την εγκεκριμένη ενέργεια.
- `executionResult` - αν APPROVED, μια συμβολοσειρά που περιγράφει το αποτέλεσμα του executor.
- `contextSnapshot` - το πλαίσιο σχολίου / σελίδας.

Αυτό το συμβάν καλύπτει όλα τα αποτελέσματα απόφασης:

- **Εγκεκριμένη + εκτελέστηκε ομαλά** -> `status: APPROVED`, ορισμένο `executedAt`, το `executionResult` είναι το μήνυμα επιτυχίας.
- **Εγκεκριμένη + ο executor απέτυχε** -> `status: EXECUTION_FAILED`, ορισμένο `executedAt`, το `executionResult` περιγράφει την αποτυχία.
- **Απορρίφθηκε** -> `status: REJECTED`, το `executedAt` είναι null, το `executionResult` είναι null.

### Κεφαλίδα

Κάθε παράδοση περιλαμβάνει μια κεφαλίδα HTTP `X-FastComments-Agent-Event` με το κανονικό όνομα συμβολοσειράς του συμβάντος (`trigger.succeeded`, κ.λπ.). Χρήσιμο αν το endpoint σας είναι ένα μοναδικό URL που διαχειρίζεται πολλαπλούς τύπους συμβάντων.

### Δείτε επίσης

- [Webhook Payloads](#webhook-payloads) για τα πλήρη schema payload ανά συμβάν.
- [Webhook Signing](#webhook-signing) για το σχήμα HMAC.
- [Webhook Retries](#webhook-retries) για τη σημασιολογία παράδοσης.