Υπάρχουν τέσσερις τύποι γεγονότων agent webhook. Κάθε γεγονός έχει μια αριθμητική τιμή enum (που χρησιμοποιείται σε payloads) και ένα κανονικό όνομα ως string (που χρησιμοποιείται στο πεδίο του περιβλήματος `event` και στο HTTP header `X-FastComments-Agent-Event`).

| Event name | Enum | Fires when |
|---|---|---|
| `trigger.succeeded` | 0 | Μία εκτέλεση agent ολοκληρώνεται με κατάσταση `SUCCESS`. |
| `trigger.failed` | 1 | Μία εκτέλεση agent ολοκληρώνεται με κατάσταση `ERROR`. |
| `approval.requested` | 2 | Μία έγκριση τοποθετείται στην κατάσταση `PENDING`. |
| `approval.decided` | 3 | Μία έγκριση μεταβαίνει σε `APPROVED`, `REJECTED`, ή `EXECUTION_FAILED`. |

### `trigger.succeeded`

Πυροδοτείται αφού η εκτέλεση του agent τελειώσει χωρίς σφάλμα. Το πεδίο `data` του payload περιλαμβάνει:

- `triggerId` - το μοναδικό ID της εκτέλεσης.
- `triggerType` - το [trigger reason enum](#triggers-overview) που ξεκίνησε την εκτέλεση.
- `status` - `SUCCESS` (string).
- `tokensUsed` - tokens που καταναλώθηκαν σε αυτή την εκτέλεση.
- `wasDryRun` - true αν ο agent ήταν σε [dry-run mode](#dry-run-mode).
- `actions` - πίνακας εγγραφών `TenantAgentAction` (βλ. [Webhook Payloads](#webhook-payloads)).
- `commentId`, `url`, `urlId` - αν ο trigger τα είχε.

Αν η εκτέλεση έκανε μηδέν ενέργειες, ο πίνακας `actions` είναι κενός - αυτή είναι μια επιτυχημένη εκτέλεση «ο agent αποφάσισε να μην κάνει τίποτα», που είναι χρήσιμο να γνωρίζεται.

### `trigger.failed`

Πυροδοτείται όταν μια εκτέλεση κάνει σφάλμα. Το σχήμα του payload είναι το ίδιο με το `trigger.succeeded`, με `status: 'ERROR'` και ένα επιπλέον πεδίο `errorMessage` που περιγράφει τι πήγε στραβά. Πιθανά σφάλματα περιλαμβάνουν αποτυχίες κλήσεων LLM, αποτυχίες διανομής εργαλείων, και εξάντληση προϋπολογισμού κατά τη διάρκεια της εκτέλεσης.

Το `actions` μπορεί ακόμα να περιέχει εγγραφές για κλήσεις εργαλείων που ολοκληρώθηκαν πριν το σφάλμα.

### `approval.requested`

Πυροδοτείται τη στιγμή που μια έγκριση τοποθετείται στην κατάσταση `PENDING`. Το payload περιλαμβάνει:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status: 'PENDING'`.
- `args` - τα ορίσματα του εργαλείου **περασμένα αυτούσια** από την κλήση LLM. Το σχήμα είναι ανά εργαλείο και δεν αποτελεί σταθερή δημόσια σύμβαση - το σχήμα μπορεί να αλλάξει καθώς προστίθενται νέα εργαλεία.
- `createdAt`.
- `justification`, `confidence` - αν τα παρείχε ο agent.
- `contextSnapshot` - το context του σχολίου / της σελίδας στο οποίο σχετίζεται η έγκριση.

Χρήσιμο για την προώθηση εκκρεμών εγκρίσεων σε κανάλι chat ops: ένα Slack bot συνδρομημένο στο `approval.requested` μπορεί να δημοσιεύσει τη δράση και τη συλλογιστική σε ένα κανάλι moderation για γρήγορη ανασκόπηση.

### `approval.decided`

Πυροδοτείται όταν μια έγκριση βγαίνει από την κατάσταση `PENDING`. Το payload περιλαμβάνει:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status` - `APPROVED`, `REJECTED`, ή `EXECUTION_FAILED`.
- `decidedBy` - το user ID του moderator που έλαβε την απόφαση.
- `decidedAt` - πότε πήρε την απόφαση.
- `executedAt` - αν είναι APPROVED, πότε η πλατφόρμα εκτέλεσε τη εγκεκριμένη ενέργεια.
- `executionResult` - αν είναι APPROVED, ένα string που περιγράφει το αποτέλεσμα του εκτελεστή.
- `contextSnapshot` - το context του σχολίου / της σελίδας.

Αυτό το γεγονός καλύπτει όλα τα αποτελέσματα απόφασης:

- **Approved + executed cleanly** -> `status: APPROVED`, το `executedAt` είναι καθορισμένο, το `executionResult` είναι το μήνυμα επιτυχίας.
- **Approved + executor failed** -> `status: EXECUTION_FAILED`, το `executedAt` είναι καθορισμένο, το `executionResult` περιγράφει την αποτυχία.
- **Rejected** -> `status: REJECTED`, το `executedAt` είναι null, το `executionResult` είναι null.

### Header

Κάθε παράδοση περιλαμβάνει ένα HTTP header `X-FastComments-Agent-Event` με το κανονικό string όνομα του γεγονότος (`trigger.succeeded`, κ.λπ.). Χρήσιμο αν το endpoint σας είναι ένα ενιαίο URL που χειρίζεται πολλούς τύπους γεγονότων.

### See also

- [Webhook Payloads](#webhook-payloads) για πλήρη σχήματα payload ανά γεγονός.
- [Webhook Signing](#webhook-signing) για το σχήμα HMAC.
- [Webhook Retries](#webhook-retries) για τις σημασιολογίες παράδοσης.