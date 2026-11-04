Αν το val σας γνωρίζει ήδη ποιος είναι ο επισκέπτης, το Secure SSO μεταφέρει αυτή την ταυτότητα στο widget ώστε να μη δει ποτέ δεύτερη σύνδεση. Δεν υπάρχουν endpoints προς κατασκευή και τίποτα προς κλήση κατά την εκτέλεση: υπολογίζετε τρεις τιμές server-side και τις περνάτε στη διαμόρφωση του widget.

Το Val Town προσφέρει σύνδεση χωρίς διαμόρφωση με το `std/oauth`, ώστε ο επισκέπτης να συνδεθεί με τον λογαριασμό Val Town που ήδη έχει. Αντικαταστήστε το με ό,τι χρησιμοποιεί η εφαρμογή σας· το κομμάτι του FastComments δεν αλλάζει.

## Κατασκευάστε το payload στον server

Το API secret υπογράφει το payload και δεν πρέπει ποτέ να φτάσει σε κώδικα browser. Εγκαταστήστε το SDK από το npm, που λειτουργεί ως έχει στο Deno runtime του Val Town:

[inline-code-attrs-start title = 'sso.ts'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import { SecureSSOPayloadBuilder } from "npm:fastcomments-sdk/server";

export function buildSSOPayload(user) {
  // Το id πρέπει να είναι σταθερό για το ίδιο άτομο, αλλιώς παίρνει νέα ταυτότητα σχολίων σε κάθε σύνδεση.
  const id = `vt-${user.id}`;

  return new SecureSSOPayloadBuilder(Deno.env.get("FASTCOMMENTS_API_SECRET"), {
    id,
    // Το email είναι υποχρεωτικό και πρέπει να είναι μοναδικό.
    email: user.email ?? `${id}@users.noreply.val.town`,
    // Το username είναι υποχρεωτικό και δεν μπορεί να είναι email.
    username: user.username ?? id,
    displayName: user.username ?? undefined,
    avatar: user.links.profileImageUrl ?? undefined,
  }).getPayload();
}
[inline-code-end]

Το `getPayload()` επιστρέφει `{ userDataJSONBase64, verificationHash, timestamp }`. Αυτές οι τρεις τιμές είναι το μόνο που φτάνει στον browser. Το secret τις υπογράφει και στη συνέχεια απορρίπτεται, ώστε τίποτα στη σελίδα να μην επιτρέπει σε έναν αναγνώστη να πλαστογραφήσει διαφορετικό χρήστη.

## Περάστε το στο widget

[inline-code-attrs-start title = 'Widget config with SSO'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import { getOAuthUserData, oauthMiddleware } from "https://esm.town/v/std/oauth/middleware.ts";

app.get("/", async (c) => {
  const session = await getOAuthUserData(c.req.raw);
  const user = session?.user;

  const config = {
    tenantId: TENANT_ID,
    urlId: "my-thread",
    ...(user
      ? { sso: { ...buildSSOPayload(user), logoutURL: "/logout" } }
      : { sso: { loginURL: "/auth/login" } }),
  };

  // ...αποδώστε το widget με αυτή τη διαμόρφωση
});

export default oauthMiddleware(app.fetch);
[inline-code-end]

Το `oauthMiddleware` προσθέτει για εσάς τα `GET /auth/login`, `GET /auth/callback` και `POST /auth/logout`. Σημειώστε ότι το logout είναι **POST**, ενώ το widget πλοηγείται στο `logoutURL` με GET, οπότε δείξτε το `logoutURL` σε ένα μικρό δικό σας route που υποβάλλει το POST.

Όταν ο επισκέπτης είναι αποσυνδεδεμένος, περάστε το `sso` με μόνο ένα `loginURL`. Το widget τότε εμφανίζει προτροπή σύνδεσης αντί για ανώνυμο πλαίσιο σχολίων.

## Πράγματα που πηγαίνουν στραβά

Το `timestamp` είναι σε epoch **milliseconds**, δεν πρέπει να είναι στο μέλλον και δεν πρέπει να είναι παλαιότερο από δύο ημέρες. Δημιουργήστε το στον server στο ίδιο request που υπολογίζει το hash. Η δημιουργία του στον browser είναι η κλασική αποτυχία: η τιμή διαφέρει από αυτή που υπογράφηκε και κάθε σχόλιο απορρίπτεται.

Ποτέ μην ορίζετε `isAdmin` ή `isModerator` από τον πάροχο ταυτότητας. Η σύνδεση με λογαριασμό Val Town δεν λέει τίποτα για το ποιος πρέπει να συντονίζει τον ιστότοπό σας.

Δείτε τον [οδηγό SSO](/guide-customizations-and-configuration.html#sso-secure) για την πλήρη λίστα πεδίων, τα threads με περιορισμό ομάδας και τα badges.