Page Reacts επιτρέπουν στους χρήστες σας να κάνουν like σε μια σελίδα ή να αντιδράσουν σε αυτήν με το δικό σας σύνολο εικόνων αντίδρασης. Το [Page Reacts widget](/guide-page-reacts.html) και το widget Floating Likes βασίζονται σε αυτά τα endpoints, και μπορείτε να τα καλέσετε εσείς οι ίδιοι για να δημιουργήσετε το δικό σας κουμπί like.

Σε αντίθεση με το υπόλοιπο του οδηγού, τα endpoints του Page Reacts είναι δημόσια. Καλούνται από τα προγράμματα περιήγησης των χρηστών σας, δεν απαιτούν κλειδί API και δεν καταναλώνουν πιστώσεις API. Κάθε αντίδραση ανήκει στον χρήστη που κάνει το αίτημα, έτσι ένας χρήστης μπορεί να προσθέσει ή να αφαιρέσει μόνο τις δικές του αντιδράσεις.

Υπάρχουν δύο σύνολα endpoints:

- `/page-reacts/v1/likes/:tenantId` - ένα μόνο «like» ανά χρήστη ανά σελίδα. Χρησιμοποιήστε τα για ένα κουμπί like.
- `/page-reacts/v2/:tenantId` - πολλαπλές αντιδράσεις ανά σελίδα, καθεμία αναγνωρίζεται από ένα σύντομο `id` που επιλέγετε (π.χ. `heart` ή `laugh`).

Και τα δύο είναι επίσης διαθέσιμα στα SDK μας ως μέρος του `PublicApi`, για παράδειγμα `getV1PageLikes`, `createV1PageReact` και `deleteV1PageReact` στο [JavaScript SDK](/guide-sdk-javascript.html).

### Showing Who Reacted

Προσθέστε `includeUserInfo=public` στα endpoints `GET` για να λαμβάνετε επίσης τους χρήστες που έκαναν like ή αντέδρασαν, με τα ονόματα χρήστη και τα avatar τους, τα πιο πρόσφατα πρώτα. Οι χρήστες που έχουν ορίσει την δραστηριότητα του προφίλ τους ως ιδιωτική εξαιρούνται, όπως και οι ανώνυμοι χρήστες. Επιστρέφονται έως 100 αντιδράσεις.

Οι χρήστες SSO δημιουργούνται με ιδιωτική δραστηριότητα προφίλ εκτός εάν το payload SSO σας ορίζει `isProfileActivityPrivate` σε `false`, οπότε ρυθμίστε το για τους χρήστες που θέλετε να εμφανίζονται σε αυτές τις λίστες.

Σε σελίδες που απαιτούν SSO, μόνο οι χρήστες SSO που μπορούν να δουν τη σελίδα μπορούν να δουν τη λίστα των αντενδείξεων. Οι άλλοι λαμβάνουν `403` με τον κωδικό `sso-required` ή `page-forbidden`.

Από τον διακομιστή σας, [`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) και [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) επιστρέφουν τις ίδιες λίστες με κλειδί API, και επίσης δέχονται `includeUserInfo=include-private` για να συμπεριλάβουν χρήστες με ιδιωτικά προφίλ.

### Identifying the User

Οι αντιδράσεις συνδέονται με τον χρήστη που κάνει το αίτημα:

- **SSO users:** περάστε την παράμετρο ερωτήματος `sso`, ορισμένη στο URI‑κωδικοποιημένο JSON του ίδιου αντικειμένου SSO που δίνετε στο widget σχολίων. Δείτε το [SSO](/guide-customizations-and-configuration.html#sso).
- **Anonymous users:** όταν δεν υπάρχει παράμετρος `sso` και δεν υπάρχει σύνδεση FastComments, ο διακομιστής εκχωρεί στο πρόγραμμα περιήγησης ένα ανώνυμο id που αποθηκεύεται στο cookie συνεδρίας FastComments. Στείλτε αιτήματα με `credentials: 'include'` ώστε το cookie να διατηρείται μεταξύ των αιτημάτων. Τα προγράμματα περιήγησης που μπλοκάρουν cookies τρίτων δεν θα διατηρήσουν το ανώνυμο id, οπότε χρησιμοποιήστε SSO όταν κάθε χρήστης πρέπει να αναγνωρίζεται αξιόπιστα.

### The urlId

`urlId` αναγνωρίζει τη σελίδα, όπως και για τα σχόλια. Χρησιμοποιήστε το ίδιο `urlId` που δίνετε στο widget σχολίων ώστε τα likes και τα σχόλια να μετρώνται στην ίδια σελίδα. Θυμηθείτε να το κωδικοποιήσετε σε URI.

[inline-code-attrs-start title = 'Παράδειγμα Κουμπιού Like'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
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