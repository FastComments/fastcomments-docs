Κάθε agent webhook υπογράφεται με HMAC-SHA256 χρησιμοποιώντας το API secret του tenant σας. Το ίδιο σχήμα υπογραφής χρησιμοποιείται για τα comment webhooks του FastComments - αν έχετε ήδη ενσωματώσει αυτά, τα agent webhooks επαναχρησιμοποιούν το ίδιο header υπογραφής και τη ροή επαλήθευσης.

### Γιατί η υπογραφή

Χωρίς υπογραφή, ένας επιτιθέμενος που γνωρίζει το URL του webhook σας θα μπορούσε να κάνει POST πλαστογραφημένα γεγονότα που φαίνονται να προέρχονται από το FastComments. Η υπογραφή σημαίνει ότι το endpoint σας μπορεί να επαληθεύει κάθε παράδοση ως αυθεντική πριν ενεργήσει.

### Πώς λειτουργούν οι υπογραφές

Για κάθε παράδοση:

1. Η πλατφόρμα αναζητεί το API secret για τον tenant + το αντιστοιχισμένο domain (βλ. [Επισκόπηση Webhooks](#webhooks-overview)).
2. Εκπέμπει το τρέχον Unix timestamp (σε milliseconds) στο header `X-FastComments-Timestamp`.
3. Υπολογίζει `HMAC-SHA256(api_secret, "${timestamp}.${raw_request_body}")` (στυλ Stripe) και εκπέμπει το αποτέλεσμα ως `sha256=<hex>` στο header `X-FastComments-Signature`.
4. Το endpoint σας διαβάζει το header του timestamp, επαναυπολογίζει το HMAC πάνω στο `${timestamp}.${body}` που έλαβε, συγκρίνει με την τιμή `sha256=<hex>` στο header της υπογραφής, και απορρίπτει τις ασυμφωνίες.

Το σώμα που υπογράφεται είναι τα **ακριβή bytes** που έστειλε η πλατφόρμα, προθεματισμένα με `${timestamp}.` - ο verifier σας πρέπει να χρησιμοποιεί το raw request body, όχι ένα επαν-σειριοποιημένο JSON string (διαφορετική σειρά κλειδιών και κενά θα δημιουργούσαν διαφορές).

### API secret

Το ίδιο API Secret που χρησιμοποιείται από τα [webhooks σχολίων](/guide-webhooks.html). Είναι ανά (tenant, domain) και διαχειρίζεται στις ρυθμίσεις API του tenant σας. Εάν αλλάξετε (rotate) το secret, θα πρέπει να επανα-deployάρετε τον verifier σας ώστε να διαβάζει τη νέα τιμή πριν την επόμενη παράδοση.

Όταν η πλατφόρμα δεν βρίσκει **καμία API secret** για το αντιστοιχισμένο domain, η παράδοση δεν πραγματοποιείται. Το log του webhook καταγράφει την αποτυχία με λόγο "no API secret".

### Παράδειγμα επαλήθευσης (Node.js)

[inline-code-attrs-start title = 'Παράδειγμα Επαλήθευσης Υπογραφής Webhook'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import crypto from 'crypto';

function verifyAgentWebhook(rawBody, signatureHeader, timestampHeader, secret) {
  const expected = 'sha256=' + crypto
    .createHmac('sha256', secret)
    .update(`${timestampHeader}.${rawBody}`)
    .digest('hex');
  return crypto.timingSafeEqual(
    Buffer.from(expected),
    Buffer.from(signatureHeader),
  );
}
[inline-code-end]

Χρησιμοποιήστε `timingSafeEqual` αντί για `===` για να αποφύγετε διαρροές μέσω timing-channel της υπογραφής.

### Τι περιέχει το υπογεγραμμένο σώμα

Ολόκληρο το envelope μαζί με το μπλοκ `data` ειδικό για το συμβάν. Δείτε [Δεδομένα Webhook](#webhook-payloads).

### Συστάσεις

- **Επαληθεύστε σε κάθε παράδοση.** Εάν το endpoint σας δέχεται μη υπογεγραμμένα αιτήματα, δεν έχετε καμία εγγύηση ακεραιότητας.
- **Απορρίψτε όταν η υπογραφή δεν ταιριάζει.** Επιστρέψτε 401 ή 403· μην επιστρέφετε 200 OK σε περίπτωση κακής υπογραφής, αλλιώς θα συγκαλύψετε επιθέσεις στα αρχεία παράδοσης.
- **Χρησιμοποιήστε HTTPS.** Οι υπογραφές προστατεύουν την ακεραιότητα· το TLS προστατεύει την εμπιστευτικότητα (τόσο το μυστικό σας όσο και το κείμενο του σχολίου στο payload).
- **Ανανεώστε τα μυστικά** όταν μέλη της ομάδας με πρόσβαση αποχωρούν, ή κατά πρόγραμμα.

### Προστασία κατά της επανάληψης (Replay protection)

Η υπογραφή από μόνη της δεν αποτρέπει επιθέσεις επανάληψης - ένας επιτιθέμενος που κατέγραψε μια πραγματική υπογεγραμμένη παράδοση μπορεί να την ξαναστείλει. Η προστασία κατά της επανάληψης είναι ευθύνη του endpoint σας:

- Χρησιμοποιήστε το πεδίο envelope `occurredAt` και απορρίψτε παραδόσεις παλαιότερες από, για παράδειγμα, 5 λεπτά.
- Χρησιμοποιήστε το `triggerId` ή το `approvalId` ως κλειδί dedup - αν το έχετε ήδη επεξεργαστεί, αγνοήστε το αντίγραφο.

### Δείτε επίσης

- [Επισκόπηση Webhooks](#webhooks-overview).
- [Δεδομένα Webhook](#webhook-payloads).
- Ο κύριος [Οδηγός Webhooks](/guide-webhooks.html) για την ευρύτερη υποδομή υπογραφών.