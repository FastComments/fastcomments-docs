Κάθε webhook agent υπογράφεται με HMAC-SHA256 χρησιμοποιώντας το API secret του tenant σας. Το ίδιο σχήμα υπογραφής χρησιμοποιείται και για τα comment webhooks του FastComments - αν τα έχετε ήδη ενσωματώσει, τα agent webhooks επαναχρησιμοποιούν το ίδιο header υπογραφής και τη ίδια ροή επαλήθευσης.

### Γιατί υπογραφή

Χωρίς υπογραφή, ένας επιτιθέμενος που γνωρίζει το URL του webhook σας θα μπορούσε να στείλει με POST πλαστά γεγονότα που φαίνονται σαν να προέρχονται από το FastComments. Η υπογραφή σημαίνει ότι το endpoint σας μπορεί να επαληθεύσει ότι κάθε παράδοση είναι γνήσια πριν δράσει.

### Πώς λειτουργούν οι υπογραφές

Για κάθε παράδοση:

1. Η πλατφόρμα αναζητά το API secret για το tenant + τον αντιστοιχισμένο domain (βλ. [Επισκόπηση Webhooks](#webhooks-overview)).
2. Εκπέμπει το τρέχον Unix timestamp (σε milliseconds) στο header `X-FastComments-Timestamp`.
3. Υπολογίζει `HMAC-SHA256(api_secret, "${timestamp}.${raw_request_body}")` (σε στυλ Stripe) και εκπέμπει το αποτέλεσμα ως `sha256=<hex>` στο header `X-FastComments-Signature`.
4. Το endpoint σας διαβάζει το header του timestamp, επανυπολογίζει το HMAC πάνω στο `${timestamp}.${body}` που έλαβε, το συγκρίνει με την τιμή `sha256=<hex>` στο header υπογραφής και απορρίπτει τις ασυμφωνίες.

Το σώμα που υπογράφεται είναι τα **ακριβή bytes** που έστειλε η πλατφόρμα, με πρόθεμα `${timestamp}.` - ο επαληθευτής σας πρέπει να χρησιμοποιήσει το raw request body, όχι μια επανασειριοποιημένη συμβολοσειρά JSON (διαφορετικά η σειρά των κλειδιών και τα κενά θα διέφεραν).

### API secret

Το ίδιο API Secret που χρησιμοποιείται από τα [comment webhooks](/guide-webhooks.html). Είναι ανά (tenant, domain) και διαχειρίζεται στις ρυθμίσεις API του tenant σας. Αν αλλάξετε το secret, θα πρέπει να επανεκδώσετε τον επαληθευτή σας ώστε να διαβάζει τη νέα τιμή πριν την επόμενη παράδοση.

Όταν η πλατφόρμα δεν βρίσκει **API secret** για τον αντιστοιχισμένο domain, η παράδοση δεν πραγματοποιείται. Το log του webhook καταγράφει την αποτυχία με αιτία "no API secret".

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

Χρησιμοποιήστε `timingSafeEqual` αντί για `===` για να αποφύγετε διαρροές της υπογραφής μέσω χρονισμού (timing-channel).

### Τι περιέχει το υπογεγραμμένο σώμα

Ο πλήρης φάκελος (envelope) μαζί με το μπλοκ `data` του συγκεκριμένου γεγονότος. Βλ. [Φορτία Webhook](#webhook-payloads).

### Συστάσεις

- **Επαληθεύστε σε κάθε παράδοση.** Αν το endpoint σας δέχεται μη υπογεγραμμένα αιτήματα, δεν έχετε καμία εγγύηση ακεραιότητας.
- **Απορρίπτετε σε ασυμφωνία υπογραφής.** Επιστρέψτε 401 ή 403· μην επιστρέφετε 200 OK σε κακή υπογραφή, αλλιώς θα καλύπτετε επιθέσεις στα logs παραδόσεων σας.
- **Χρησιμοποιήστε HTTPS.** Οι υπογραφές προστατεύουν την ακεραιότητα· το TLS προστατεύει την εμπιστευτικότητα (και το secret σας και το κείμενο των σχολίων στο payload).
- **Αλλάζετε τα secrets** όταν φεύγουν μέλη της ομάδας με πρόσβαση ή σύμφωνα με πρόγραμμα.

### Προστασία από replay

Η υπογραφή από μόνη της δεν αποτρέπει επιθέσεις replay - ένας επιτιθέμενος που κατέγραψε μια πραγματική υπογεγραμμένη παράδοση μπορεί να την ξαναστείλει. Η προστασία από replay είναι ευθύνη του endpoint σας:

- Χρησιμοποιήστε το πεδίο `occurredAt` του envelope και απορρίπτετε παραδόσεις παλαιότερες από, ας πούμε, 5 λεπτά.
- Χρησιμοποιήστε το `triggerId` ή το `approvalId` ως κλειδί αποδιομοπαλληκής (dedup) - αν το έχετε ήδη επεξεργαστεί, αγνοήστε το διπλότυπο.

### Δείτε επίσης

- [Επισκόπηση Webhooks](#webhooks-overview).
- [Φορτία Webhook](#webhook-payloads).
- Ο κύριος [οδηγός Webhooks](/guide-webhooks.html) για τη ευρύτερη υποδομή υπογραφής.