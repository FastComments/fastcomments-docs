Το widget είναι μια ετικέτα `script` και ένα στοιχείο container, έτσι ενσωματώνεται σε ό,τι ήδη αποδίδει το val σας. Αυτό το παράδειγμα χρησιμοποιεί Hono JSX, το οποίο είναι αυτό που χρησιμοποιούν τα HTTP templates του Val Town.

[inline-code-attrs-start title = 'Widget σχολίων σε ένα HTTP val'; type='javascript' inline-code-attrs-end]
[inline-code-start]
/** @jsxImportSource npm:hono@4/jsx */
import { Hono } from "npm:hono@4";

const app = new Hono();

app.get("/:slug", (c) => {
  const slug = c.req.param("slug");
  const url = new URL(c.req.path, c.req.url).toString();

  const config = JSON.stringify({
    tenantId: "demo",
    urlId: slug,
    url,
  });

  return c.html(
    <html>
      <body>
        <h1>{slug}</h1>
        <div id="fastcomments-widget"></div>
        <script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
        <script
          dangerouslySetInnerHTML={{
            __html:
              `window.FastCommentsUI(document.getElementById("fastcomments-widget"), ${config});`,
          }}
        />
      </body>
    </html>,
  );
});

export default app.fetch;
[inline-code-end]

## Επιλέξτε ένα urlId πριν το εκδώσετε

`urlId` καθορίζει σε ποιο νήμα θα τοποθετηθεί ένα σχόλιο. Αν το αφήσετε ακαθορισμένο, προεπιλέγεται μια καθαρισμένη έκδοση του τρέχοντος URL της σελίδας, που είναι ακριβώς το στοιχείο που αλλάζει στο Val Town: ένα val έχει ένα μακρύ hostname `*.web.val.run` μέχρι να διεκδικήσετε ένα υποτομέα, τα κλαδιά παίρνουν τα δικά τους URL και η μετονομασία μιας σελίδας αλλάζει τη διαδρομή. Κάθε παραλλαγή γίνεται σιωπηρά ένα ξεχωριστό, κενό νήμα, και το σύμπτωμα εμφανίζεται ως «τα σχόλιά μου εξαφανίστηκαν».

Ορίστε το σε κάτι σταθερό που ελέγχετε, όπως το slug της ανάρτησης ή ένα αναγνωριστικό βάσης δεδομένων, όπως παραπάνω. Περνάτε επίσης το `url`, ώστε τα email ειδοποιήσεων και τα εργαλεία διαχείρισης να μπορούν να συνδέονται πίσω στη πραγματική σελίδα.

## Διατήρηση σχολίων χωρίς JavaScript

Το FastComments αποδίδει ένα πλήρες νήμα διακομιστή, το οποίο ένα val μπορεί να ενσωματώσει σε ένα μπλοκ `<noscript>`:

[inline-code-attrs-start title = 'Εναλλακτική λύση χωρίς JavaScript'; type='html' inline-code-attrs-end]
[inline-code-start]
<noscript>
  <iframe src="https://fastcomments.com/ssr/comments?tenantId=demo&urlId=POST_SLUG&url=PAGE_URL"
          title="FastComments" width="100%" height="1500px" frameborder="0"
          style="width: 1px !important; min-width: 100% !important; border: none !important;"></iframe>
</noscript>
[inline-code-end]

Κωδικοποιήστε τις παραμέτρους σε URL. Η έκδοση διακομιστή υποστηρίζει ανώνυμη και συνδεδεμένη σχολιασμό, SSO, και ένθετες απαντήσεις.