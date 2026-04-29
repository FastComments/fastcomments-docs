Υπάρχουν τέσσερις τύποι συμβάντων webhook του agent. Κάθε συμβάν έχει μια αριθμητική τιμή enum (που χρησιμοποιείται στα payloads) και ένα κανoνικό string όνομα (που χρησιμοποιείται στο πεδίο `event` του φακέλου και στην HTTP κεφαλίδα `X-FastComments-Agent-Event`).

| Event name | Enum | Fires when |
|---|---|---|
| `trigger.succeeded` | 0 | Μια εκτέλεση του agent ολοκληρώνεται με κατάσταση `SUCCESS`. |
| `trigger.failed` | 1 | Μια εκτέλεση του agent ολοκληρώνεται με κατάσταση `ERROR`. |
| `approval.requested` | 2 | Μια έγκριση τοποθετείται σε κατάσταση `PENDING`. |
| `approval.decided` | 3 | Μια έγκριση μεταβαίνει σε `APPROVED`, `REJECTED`, ή `EXECUTION_FAILED`. |

### `trigger.succeeded`

Εκδηλώνεται μετά την ολοκλήρωση της εκτέλεσης του agent χωρίς σφάλμα. Το πεδίο `data` του payload περιλαμβάνει:

- `triggerId` - το μοναδικό ID εκτέλεσης.
- `triggerType` - το [trigger reason enum](#triggers-overview) που ξεκίνησε την εκτέλεση.
- `status` - `SUCCESS` (string).
- `tokensUsed` - tokens που καταναλώθηκαν σε αυτή την εκτέλεση.
- `wasDryRun` - true αν ο agent ήταν σε [dry-run mode](#dry-run-mode).
- `actions` - πίνακας εγγραφών `TenantAgentAction` (βλ. [Webhook Payloads](#webhook-payloads)).
- `commentId`, `url`, `urlId` - εάν ο trigger τα είχε.

Αν η εκτέλεση δεν έκανε καμία ενέργεια, ο πίνακας `actions` είναι κενός - πρόκειται για μια επιτυχημένη "the agent decided to do nothing" εκτέλεση, κάτι που είναι χρήσιμο να γνωρίζετε.

### `trigger.failed`

Εκδηλώνεται όταν μια εκτέλεση παράγει σφάλμα. Το σχήμα του payload είναι ίδιο με του `trigger.succeeded`, με `status: 'ERROR'` και πρόσθετο πεδίο `errorMessage` που περιγράφει τι πήγε στραβά. Πιθανά σφάλματα περιλαμβάνουν αποτυχίες κλήσεων LLM, αποτυχίες αποστολής σε εργαλεία και εξάντληση του προϋπολογισμού κατά τη διάρκεια της εκτέλεσης.

`actions` μπορεί να περιέχει ακόμα εγγραφές για κλήσεις εργαλείων που ολοκληρώθηκαν πριν το σφάλμα.

### `approval.requested`

Εκδηλώνεται τη στιγμή που μια έγκριση τοποθετείται στην κατάσταση `PENDING`. Το payload περιλαμβάνει:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status: 'PENDING'`.
- `args` - τα επιχειρήματα του εργαλείου **προωθούνται αυτούσια** από την κλήση LLM. Το σχήμα είναι ανά εργαλείο και δεν αποτελεί σταθερή δημόσια συμφωνία - το σχήμα μπορεί να αλλάξει καθώς προστίθενται νέα εργαλεία.
- `createdAt`.
- `justification`, `confidence` - αν τα παρείχε ο agent.
- `contextSnapshot` - το context του σχολίου/της σελίδας στο οποίο σχετίζεται η έγκριση.

Χρήσιμο για προώθηση εκκρεμών εγκρίσεων σε κανάλι chat ops: ένα Slack bot εγγεγραμμένο στο `approval.requested` μπορεί να δημοσιεύει την ενέργεια και τα επιχειρήματα σε ένα κανάλι moderation για άμεση επισκόπηση.

### `approval.decided`

Εκδηλώνεται όταν μια έγκριση βγαίνει από την κατάσταση `PENDING`. Το payload περιλαμβάνει:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status` - `APPROVED`, `REJECTED`, ή `EXECUTION_FAILED`.
- `decidedBy` - το user ID του moderator που πήρε την απόφαση.
- `decidedAt` - πότε πήρε την απόφαση.
- `executedAt` - αν APPROVED, πότε η πλατφόρμα εκτέλεσε την εγκεκριμένη ενέργεια.
- `executionResult` - αν APPROVED, μια συμβολοσειρά που περιγράφει το αποτέλεσμα του executor.
- `contextSnapshot` - το context του σχολίου/της σελίδας.

Αυτό το συμβάν καλύπτει όλα τα αποτελέσματα της απόφασης:

- **Έγκριση + εκτέλεση χωρίς σφάλματα** -> `status: APPROVED`, το `executedAt` ορισμένο, το `executionResult` είναι το μήνυμα επιτυχίας.
- **Έγκριση + αποτυχία του executor** -> `status: EXECUTION_FAILED`, το `executedAt` ορισμένο, το `executionResult` περιγράφει την αποτυχία.
- **Απορρίφθηκε** -> `status: REJECTED`, το `executedAt` είναι null, το `executionResult` είναι null.

### Header

Κάθε παράδοση περιλαμβάνει την HTTP κεφαλίδα `X-FastComments-Agent-Event` με το κανoνικό string όνομα του συμβάντος (`trigger.succeeded`, κ.λπ.). Χρήσιμο αν το endpoint σας είναι ένα ενιαίο URL που χειρίζεται πολλούς τύπους συμβάντων.

### See also

- [Webhook Payloads](#webhook-payloads) για τα πλήρη σχήματα payload ανά συμβάν.
- [Webhook Signing](#webhook-signing) για το σχήμα HMAC.
- [Webhook Retries](#webhook-retries) για τη συμπεριφορά παράδοσης.