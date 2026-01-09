[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO χρησιμοποιεί κρυπτογράφηση HMAC-SHA256 ως τον μηχανισμό υλοποίησης του SSO. Πρώτα θα δούμε την συνολική αρχιτεκτονική, θα παρέχουμε παραδείγματα και λεπτομερή βήματα.

Υπάρχει επίσης κάποια τεκμηρίωση σχετικά με τη μετανάστευση από άλλους παρόχους με παρόμοιους μηχανισμούς SSO, και τις διαφορές.

Η ροή έχει ως εξής:

<div class="screenshot white-bg">
    <div class="title">Διαδικασία Secure SSO</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Διάγραμμα Secure SSO" />
</div>

Εφόσον το Secure SSO περιλαμβάνει ανάπτυξη πλήρους στοίβας, πλήρη λειτουργικά παραδείγματα κώδικα σε Java/Spring, NodeJS/Express, και vanilla PHP βρίσκονται <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">στο GitHub</a>.

Αν και χρησιμοποιούμε ExpressJS στο παράδειγμα NodeJS και Spring στο παράδειγμα Java, δεν απαιτούνται πλαίσια/βιβλιοθήκες σε αυτά τα runtime για να υλοποιήσετε το FastComments SSO - οι εγγενείς βιβλιοθήκες κρυπτογραφίας αρκούν.

Δεν χρειάζεται να γράψετε νέα API endpoints με το FastComments SSO. Απλά κρυπτογραφήστε τις πληροφορίες του χρήστη χρησιμοποιώντας το secret key σας και περάστε το payload στο comment widget.

#### Get Your API Secret Key

Το μυστικό API σας μπορεί να ανακτηθεί από <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">αυτή τη σελίδα</a>. Μπορείτε επίσης να βρείτε αυτή τη σελίδα πηγαίνοντας στο My Account, κάνοντας κλικ στο πλακίδιο API/SSO, και στη συνέχεια κάνοντας κλικ στο "Get API Secret Key".

#### Comment Widget Parameters

Υψηλού επιπέδου τεκμηρίωση API για το comment widget μπορεί να βρεθεί <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">εδώ</a>.

Ας δούμε λεπτομερέστερα τι σημαίνουν αυτές οι παράμετροι.

Το comment widget παίρνει ένα αντικείμενο ρυθμίσεων - ήδη περνάτε αυτό εάν χρησιμοποιείτε το FastComments για να δώσετε το customer id σας (που ονομάζεται tenantId).

Για να ενεργοποιήσετε το SSO, περάστε ένα νέο αντικείμενο "sso", το οποίο πρέπει να έχει τις ακόλουθες παραμέτρους. Οι τιμές πρέπει να δημιουργούνται από το server.

- userDataJSONBase64: Τα δεδομένα του χρήστη σε μορφή JSON, τα οποία στη συνέχεια κωδικοποιούνται σε Base64.
- verificationHash: Το HMAC-SHA256 hash που δημιουργείται από το UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Epoch timestamp, σε **milliseconds**. Δεν πρέπει να είναι στο μέλλον, ή περισσότερο από δύο μέρες στο παρελθόν.
- loginURL: Ένα URL που το comment widget μπορεί να εμφανίσει για να συνδεθεί ο χρήστης.
- logoutURL: Ένα URL που το comment widget μπορεί να εμφανίσει για να αποσυνδεθεί ο χρήστης.
- loginCallback: Όταν παρέχεται αντί του login URL, μια συνάρτηση που το comment widget θα καλεί όταν γίνει κλικ στο κουμπί σύνδεσης.
- logoutCallback: Όταν παρέχεται αντί του logout URL, μια συνάρτηση που το comment widget θα καλεί όταν γίνει κλικ στο κουμπί αποσύνδεσης.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### Το Αντικείμενο Χρήστη

Το αντικείμενο User περιέχει το ακόλουθο σχήμα:
[inline-code-attrs-start title = 'Το Αντικείμενο Χρήστη'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Απαιτείται. Μέγιστο 1.000 χαρακτήρες. **/
    id: string;
    /** Απαιτείται. Μέγιστο 1.000 χαρακτήρες. Σημείωση: Πρέπει να είναι μοναδικό. **/
    email: string;
    /** Απαιτείται. Μέγιστο 1.000 χαρακτήρες. Σημείωση: Το όνομα χρήστη δεν μπορεί να είναι διεύθυνση email. Δεν χρειάζεται να είναι μοναδικό. **/
    username: string;
    /** Προαιρετικό. Μέγιστο 3.000 χαρακτήρες για URLs. Από προεπιλογή χρησιμοποιείται το gravatar βάσει του email. Υποστηρίζει εικόνες κωδικοποιημένες σε base64, οπότε το όριο σε αυτή την περίπτωση είναι 50.000 χαρακτήρες. **/ 
    avatar?: string;
    /** Προαιρετικό. Προεπιλογή false. **/
    optedInNotifications?: boolean;
    /** Προαιρετικό. Προεπιλογή false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Προαιρετικό. Μέγιστο 100 χαρακτήρες. Αυτή η ετικέτα θα εμφανίζεται δίπλα στο όνομά τους. Από προεπιλογή είναι Διαχειριστής/Συντονιστής όταν εφαρμόζεται. **/
    displayLabel?: string;
    /** Προαιρετικό. Μέγιστο 500 χαρακτήρες. Αυτό θα εμφανίζεται αντί του ονόματος χρήστη. **/
    displayName?: string;
    /** Προαιρετικό. Μέγιστο 2.000 χαρακτήρες. Το όνομα του χρήστη θα συνδέεται με αυτό. **/
    websiteUrl?: string;
    /** Προαιρετικό. Έως 100 ομάδες ανά χρήστη. Ένα id ομάδας δεν μπορεί να είναι μεγαλύτερο από 50 χαρακτήρες. **/
    groupIds?: string[];
    /** Προαιρετικό. Δηλώνει ότι ο χρήστης είναι διαχειριστής. **/
    isAdmin?: boolean;
    /** Προαιρετικό. Δηλώνει ότι ο χρήστης είναι συντονιστής. **/
    isModerator?: boolean;
    /** Προαιρετικό, προεπιλογή true. Ορίστε σε false για να ενεργοποιήσετε την καρτέλα "activity" στο προφίλ του χρήστη. **/
    isProfileActivityPrivate?: boolean;
    /** Προαιρετικό, προεπιλογή false. Ορίστε σε true για να απενεργοποιήσετε τα σχόλια στο προφίλ. **/
    isProfileCommentsPrivate?: boolean;
    /** Προαιρετικό, προεπιλογή false. Ορίστε σε true για να απενεργοποιήσετε την απευθείας αποστολή μηνυμάτων σε αυτόν τον χρήστη. **/
    isProfileDMDisabled?: boolean;
}
[inline-code-end]

#### Συντονιστές και Διαχειριστές

Για τους admins και τους moderators, περάστε τα αντίστοιχα flags `isAdmin` ή `isModerator` στο αντικείμενο `SSOUser`.

#### Ειδοποιήσεις

Για να ενεργοποιήσετε ή να απενεργοποιήσετε τις ειδοποιήσεις, ορίστε την τιμή του `optedInNotifications` σε `true` ή `false` αντίστοιχα. Την πρώτη φορά που ο χρήστης φορτώνει τη σελίδα με αυτή την τιμή στο SSO payload, οι ρυθμίσεις ειδοποιήσεων του θα ενημερωθούν.

Επιπλέον, εάν θέλετε οι χρήστες να λαμβάνουν email ειδοποιήσεων για δραστηριότητα σε σελίδες στις οποίες έχουν εγγραφεί (σε αντίθεση με μόνο in-app ειδοποιήσεις), τότε ορίστε το `optedInSubscriptionNotifications` σε `true`.

#### VIP Users & Special Labels

Μπορείτε να εμφανίσετε μια ειδική ετικέτα δίπλα στο όνομα του χρήστη χρησιμοποιώντας το προαιρετικό πεδίο "displayLabel".

#### Μη αυθεντικοποιημένοι χρήστες

Για να αναπαραστήσετε έναν μη αυθεντικοποιημένο χρήστη, απλά μην συμπληρώσετε τα userDataJSONBase64, verificationHash, ή timestamp. Παρέχετε ένα loginURL.

Αυτοί οι χρήστες δεν θα μπορούν να σχολιάσουν, και αντ' αυτού θα τους παρουσιαστεί ένα μήνυμα σύνδεσης (μήνυμα, σύνδεσμος, ή κουμπί, ανάλογα με τη διαμόρφωση).

#### Άμεσα Παραδείγματα για Σειριοποίηση και Δημιουργία Hash των Δεδομένων Χρήστη

Περισσότερες λεπτομέρειες και παραδείγματα <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">εδώ</a> (js), <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">εδώ</a> (java) και <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">εδώ</a> (php).

Κατανοούμε ότι οποιαδήποτε ενσωμάτωση μπορεί να είναι μια περίπλοκη και επώδυνη διαδικασία. Μη διστάσετε να επικοινωνήσετε με τον εκπρόσωπό σας ή να χρησιμοποιήσετε τη <a href="https://fastcomments.com/auth/my-account/help" target="_blank">σελίδα υποστήριξης</a>.