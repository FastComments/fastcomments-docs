Page Reacts επιτρέπει στους χρήστες σας να κάνουν like σε μια σελίδα ή να αντιδράσουν σε αυτήν με το δικό σας σύνολο εικόνων αντίδρασης. Το [Page Reacts widget](/guide-page-reacts.html) και το widget Floating Likes βασίζονται σε αυτά τα endpoints, και μπορείτε να τα καλέσετε εσείς οι ίδιοι για να δημιουργήσετε το δικό σας κουμπί like.

Σε αντίθεση με το υπόλοιπο του οδηγού, τα endpoints του Page Reacts είναι δημόσια. Καλούνται από τα προγράμματα περιήγησης των χρηστών σας, δεν απαιτούν κλειδί API και δεν κοστίζουν μονάδες API. Κάθε αντίδραση ανήκει στον χρήστη που κάνει το αίτημα, έτσι ένας χρήστης μπορεί να προσθέσει ή να αφαιρέσει μόνο τις δικές του.

Υπάρχουν δύο σύνολα endpoints:

- `/page-reacts/v1/likes/:tenantId` - ένα μόνο "like" ανά χρήστη ανά σελίδα. Χρησιμοποιήστε τα για ένα κουμπί like.
- `/page-reacts/v2/:tenantId` - πολλαπλές αντιδράσεις ανά σελίδα, καθεμία αναγνωρίζεται από ένα σύντομο `id` που επιλέγετε (π.χ. `heart` ή `laugh`).

Και τα δύο είναι επίσης διαθέσιμα στα SDK μας ως μέρος του `PublicApi`, για παράδειγμα `getV1PageLikes`, `createV1PageReact` και `deleteV1PageReact` στο [JavaScript SDK](/guide-sdk-javascript.html).

### Αναγνώριση του Χρήστη

Οι αντιδράσεις συνδέονται με τον χρήστη που κάνει το αίτημα:

- **SSO users:** περάστε την παράμετρο ερωτήματος `sso`, η οποία πρέπει να είναι το URI‑κωδικοποιημένο JSON του ίδιου αντικειμένου SSO που δίνετε στο widget σχολίων. Δείτε το [SSO](/guide-customizations-and-configuration.html#sso).
- **Anonymous users:** όταν δεν υπάρχει παράμετρος `sso` και δεν υπάρχει σύνδεση FastComments, ο διακομιστής εκχωρεί στον περιηγητή ένα ανώνυμο id που αποθηκεύεται στο cookie συνεδρίας FastComments. Στείλτε αιτήματα με `credentials: 'include'` ώστε το cookie να διατηρείται μεταξύ των αιτημάτων. Οι περιηγητές που μπλοκάρουν τα cookies τρίτων δεν θα διατηρήσουν το ανώνυμο id, επομένως χρησιμοποιήστε SSO όταν κάθε χρήστης πρέπει να αναγνωρίζεται αξιόπιστα.

### Το urlId

`urlId` αναγνωρίζει τη σελίδα, όπως και για τα σχόλια. Χρησιμοποιήστε το ίδιο `urlId` που δίνετε στο widget σχολίων ώστε τα likes και τα σχόλια να μετρώνται στην ίδια σελίδα. Θυμηθείτε να το κωδικοποιήσετε με URI.

[inline-code-attrs-start title = 'Παράδειγμα Κουμπιού Μου Αρέσει'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId = 'demo';
const urlId = 'https://example.com/my-article';
// Optional, for SSO users. The same object you give the comment widget's "sso" option.
const sso = null;

function likesUrl() {
    let url = `https://fastcomments.com/page-reacts/v1/likes/${tenantId}?urlId=${encodeURIComponent(urlId)}`;
    if (sso) {
        url += '&sso=' + encodeURIComponent(JSON.stringify(sso));
    }
    return url;
}

async function getLikes() {
    const response = await fetch(likesUrl(), {credentials: 'include'});
    return response.json(); // {status, likeCount, didLike, commentCount, urlIdWS}
}

async function like() {
    await fetch(likesUrl(), {method: 'POST', credentials: 'include'});
}

async function unlike() {
    await fetch(likesUrl(), {method: 'DELETE', credentials: 'include'});
}
[inline-code-end]

---