A val είναι ένας φυσικός δέκτης webhook: διαθέτει σταθερό URL, μπορεί να επαληθεύσει μια υπογραφή και έχει ενσωματωμένη αποθήκευση SQLite και blob.

FastComments υπογράφει `${timestamp}.${body}` με το μυστικό API του λογαριασμού σας και στέλνει δύο κεφαλίδες:

[inline-code-attrs-start title = 'Κεφαλίδες Webhook'; type='text' inline-code-attrs-end]
[inline-code-start]
X-FastComments-Timestamp: 1789004710        unix seconds, not milliseconds
X-FastComments-Signature: sha256=<hex>
[inline-code-end]

Η μέθοδος μεταφέρει το γεγονός: **PUT** για ένα δημιουργημένο ή ενημερωμένο σχόλιο, **DELETE** για ένα διαγραμμένο.

[inline-code-attrs-start title = 'Επαλήθευση παράδοσης'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import { createHmac, timingSafeEqual } from "node:crypto";

async function receive(c) {
  // The exact bytes that arrived. Do NOT use c.req.json() and re-serialize.
  const rawBody = await c.req.raw.text();
  const timestamp = c.req.raw.headers.get("X-FastComments-Timestamp");
  const signature = c.req.raw.headers.get("X-FastComments-Signature");

  if (!timestamp || !signature) return new Response("Missing headers", { status: 400 });

  // Reject stale deliveries so a captured request cannot be replayed later.
  if (Math.abs(Math.floor(Date.now() / 1000) - Number(timestamp)) > 300) {
    return new Response("Timestamp outside window", { status: 400 });
  }

  const expected = "sha256=" + createHmac("sha256", Deno.env.get("FASTCOMMENTS_API_SECRET"))
    .update(`${timestamp}.${rawBody}`)
    .digest("hex");

  const a = new TextEncoder().encode(signature);
  const b = new TextEncoder().encode(expected);
  if (a.length !== b.length || !timingSafeEqual(a, b)) {
    return new Response("Signature mismatch", { status: 401 });
  }

  // ...handle JSON.parse(rawBody)
  return Response.json({ received: true });
}

app.put("/", receive);
app.delete("/", receive);
[inline-code-end]

## Δύο πράγματα που προκαλούν προβλήματα

**Επαληθεύστε τα ακατέργαστα byte.** Η ανάλυση του JSON και η επανα-σειριοποίησή του αλλάζει τη σειρά των κλειδιών και τα κενά, έτσι το hash διαφέρει και κάθε παράδοση αποτυγχάνει χωρίς προφανή λόγο. Αυτό είναι ο συνηθισμένος λόγος που ένας δέκτης webhook «απλώς δεν λειτουργεί».

**Συγκρίνετε σε σταθερό χρόνο.** Ένα απλό `===` στην υπογραφή διαρρέει πόσα byte ταιριάζουν, κάτι που αρκεί για να παραχθεί ένα byte τη φορά.

## Διαχείριση συμβάντων

Απαντήστε γρήγορα. Το FastComments κάνει επανεγγραφές σε μη-2xx απαντήσεις, και ένα endpoint που συνεχίζει να αποτυγχάνει απενεργοποιείται αυτόματα μετά από κάποιο χρόνο, επομένως εκτελέστε την πραγματική εργασία μετά την απάντηση αντί εντός της απάντησης.

Κάντε αυτή τη λειτουργία αμετάβλητη (idempotent) με βάση το αναγνωριστικό του σχολίου. Μια επανεγγραφή υπογράφεται ξανά με νέο χρονικό σήμα, και το ίδιο αναγνωριστικό σχολίου φτάνει ξανά σε επεξεργασία και διαγραφή, οπότε δεν υπάρχει κάτι σταθερό για απο-διπλοεγγραφή.