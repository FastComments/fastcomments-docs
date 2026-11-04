Τα Page Reacts επιτρέπουν στους χρήστες σας να κάνουν like σε μια σελίδα ή να αντιδράσουν σε αυτήν με το δικό σας σύνολο εικόνων αντιδράσεων. Το [widget Page Reacts](/guide-page-reacts.html) και το widget Floating Likes βασίζονται σε αυτά τα endpoints, και μπορείτε να τα καλέσετε οι ίδιοι για να φτιάξετε το δικό σας κουμπί like.

Σε αντίθεση με το υπόλοιπο αυτού του οδηγού, τα endpoints των Page Reacts είναι δημόσια. Καλούνται από τα browsers των χρηστών σας, δεν απαιτούν API key και δεν κοστίζουν API credits. Κάθε αντίδραση ανήκει στον χρήστη που κάνει το αίτημα, οπότε ένας χρήστης μπορεί να προσθέσει ή να αφαιρέσει μόνο τη δική του.

Υπάρχουν δύο σύνολα endpoints:

- `/page-reacts/v1/likes/:tenantId` - ένα "like" ανά χρήστη ανά σελίδα. Χρησιμοποιήστε αυτά για ένα κουμπί like.
- `/page-reacts/v2/:tenantId` - πολλαπλές αντιδράσεις ανά σελίδα, καθεμία αναγνωρισμένη από ένα σύντομο `id` που επιλέγετε εσείς (για παράδειγμα `heart` ή `laugh`).

Και τα δύο είναι διαθέσιμα και στα SDK μας ως μέρος του `PublicApi`, για παράδειγμα `getV1PageLikes`, `createV1PageReact` και `deleteV1PageReact` στο [JavaScript SDK](/guide-sdk-javascript.html).

### Εμφάνιση του Ποιος Αντέδρασε

Περάστε `includeUserInfo=public` στα `GET` endpoints για να λάβετε επίσης τους χρήστες που έκαναν like ή αντέδρασαν, με τα ονόματα χρήστη και τα avatars τους, με τους πιο πρόσφατους πρώτους. Οι χρήστες που έχουν κάνει ιδιωτική τη δραστηριότητα του προφίλ τους εξαιρούνται, όπως και οι ανώνυμοι χρήστες. Επιστρέφονται έως 100 αντιδράσεις.

Οι χρήστες SSO δημιουργούνται με ιδιωτική δραστηριότητα προφίλ, εκτός αν το SSO payload σας ορίσει το `isProfileActivityPrivate` σε `false`, οπότε ορίστε το για τους χρήστες που θέλετε να εμφανίζονται σε αυτές τις λίστες.

Σε σελίδες που απαιτούν SSO, μόνο χρήστες SSO που μπορούν να δουν τη σελίδα μπορούν να εμφανίσουν ποιος αντέδρασε. Οι άλλοι λαμβάνουν `403` με τον κωδικό `sso-required` ή `page-forbidden`.

Από τον server σας, τα [`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) και [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) επιστρέφουν τις ίδιες λίστες με API key, και δέχονται επίσης `includeUserInfo=include-private` για να συμπεριλάβουν χρήστες με ιδιωτικά προφίλ.

### Αναγνώριση του Χρήστη

Οι αντιδράσεις συνδέονται με τον χρήστη που κάνει το αίτημα:

- **Χρήστες SSO:** περάστε την παράμετρο query `sso`, ορισμένη στο URI encoded JSON του ίδιου SSO αντικειμένου που δίνετε στο widget σχολίων. Δείτε [SSO](/guide-customizations-and-configuration.html#sso).
- **Ανώνυμοι χρήστες:** όταν δεν υπάρχει παράμετρος `sso` ούτε login στο FastComments, ο server αναθέτει στο browser ένα ανώνυμο id που αποθηκεύεται στο session cookie του FastComments. Στείλτε τα αιτήματα με `credentials: 'include'` ώστε το cookie να διατηρείται μεταξύ των αιτημάτων. Browsers που μπλοκάρουν third-party cookies δεν θα διατηρήσουν το ανώνυμο id, οπότε χρησιμοποιήστε SSO όταν κάθε χρήστης πρέπει να αναγνωρίζεται αξιόπιστα.

### Το urlId

Το `urlId` αναγνωρίζει τη σελίδα, όπως ακριβώς κάνει και για τα σχόλια. Χρησιμοποιήστε το ίδιο `urlId` που δίνετε στο widget σχολίων ώστε τα likes και τα σχόλια να μετριούνται στην ίδια σελίδα. Θυμηθείτε να το κωδικοποιήσετε με URI.

[inline-code-attrs-start title = 'Παράδειγμα Κουμπιού Like'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId = 'demo';
const urlId = 'https://example.com/my-article';
// Προαιρετικό, για χρήστες SSO. Το ίδιο αντικείμενο που δίνετε στην επιλογή "sso" του widget σχολίων.
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