[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

Το FastComments Secure SSO χρησιμοποιεί κρυπτογράφηση HMAC-SHA256 ως μηχανισμό για την υλοποίηση του SSO. Πρώτα θα περιγράψουμε τη συνολική αρχιτεκτονική, θα δώσουμε παραδείγματα και λεπτομερή βήματα.

Υπάρχει επίσης κάποια τεκμηρίωση σχετικά με τη μετεγκατάσταση από άλλους παρόχους με παρόμοιους μηχανισμούς SSO, και οι διαφορές.

Η ροή μοιάζει ως εξής:

<div class="screenshot white-bg">
    <div class="title">Secure SSO Flow</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Secure SSO Diagram" />
</div>

Εφόσον το Secure SSO περιλαμβάνει full-stack ανάπτυξη, πλήρη λειτουργικά παραδείγματα κώδικα σε Java/Spring, NodeJS/Express, και vanilla PHP βρίσκονται αυτήν τη στιγμή <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">στο GitHub</a>.

Παρόλο που χρησιμοποιούμε ExpressJS στο παράδειγμα NodeJS και Spring στο παράδειγμα Java, δεν απαιτούνται frameworks/βιβλιοθήκες σε αυτά τα runtimes για την υλοποίηση του FastComments SSO - τα εγγενή πακέτα κρυπτογράφησης λειτουργούν.

Δεν χρειάζεται να γράψετε νέα API endpoints με το FastComments SSO. Απλώς κρυπτογραφήστε τις πληροφορίες του χρήστη χρησιμοποιώντας το μυστικό κλειδί σας και περάστε το payload στο comment widget.

#### Get Your API Secret Key

Το API Secret σας μπορεί να ανακτηθεί από <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">αυτή τη σελίδα</a>. Μπορείτε επίσης να βρείτε αυτή τη σελίδα πηγαίνοντας στο My Account, κάνοντας κλικ στο πλακίδιο API/SSO, και στη συνέχεια κάνοντας κλικ στο "Get API Secret Key".

#### Comment Widget Parameters

Η υψηλού επιπέδου τεκμηρίωση API για το comment widget βρίσκεται <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">εδώ</a>.

Ας δούμε με περισσότερες λεπτομέρειες τι σημαίνουν αυτοί οι παράμετροι.

Το comment widget παίρνει ένα αντικείμενο διαμόρφωσης - ήδη το περνάτε αν χρησιμοποιείτε το FastComments για να περάσετε το tenantId.

Για να ενεργοποιήσετε το SSO, περάστε ένα νέο αντικείμενο "sso", το οποίο πρέπει να έχει τις ακόλουθες παραμέτρους. Οι τιμές πρέπει να δημιουργούνται στην πλευρά του server.

- userDataJSONBase64: Τα δεδομένα του χρήστη σε μορφή JSON, τα οποία στη συνέχεια κωδικοποιούνται σε Base64.
- verificationHash: Το HMAC-SHA256 hash που δημιουργείται από UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Epoch timestamp, σε **milliseconds**. Δεν πρέπει να είναι στο μέλλον, ή περισσότερο από δύο ημέρες στο παρελθόν.
- loginURL: Ένα URL που το comment widget μπορεί να εμφανίσει για να συνδέσει τον χρήστη.
- logoutURL: Ένα URL που το comment widget μπορεί να εμφανίσει για να αποσυνδέσει τον χρήστη.
- loginCallback: Όταν παρέχεται αντί για το login URL, μια συνάρτηση που το comment widget θα καλεί όταν κάνετε κλικ στο κουμπί login.
- logoutCallback: Όταν παρέχεται αντί για το logout URL, μια συνάρτηση που το comment widget θα καλεί όταν κάνετε κλικ στο κουμπί logout.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### The User Object

The User object contains the following schema:
[inline-code-attrs-start title = 'Το Αντικείμενο Χρήστη'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Απαιτείται. Μέγιστο 1k χαρακτήρες. **/
    id: string;
    /** Απαιτείται. Μέγιστο 1k χαρακτήρες. Σημείωση: Πρέπει να είναι μοναδικό. **/
    email: string;
    /** Απαιτείται. Μέγιστο 1k χαρακτήρες. Σημείωση: Το username δεν μπορεί να είναι email. Δεν χρειάζεται να είναι μοναδικό. **/
    username: string;
    /** Προαιρετικό. Μέγιστο 3k χαρακτήρες για URLs. Προεπιλογή είναι από gravatar βάσει email. Υποστηρίζει 64-κωδικοποιημένες εικόνες, οπότε το όριο είναι 50k χαρακτήρες. **/ 
    avatar?: string;
    /** Προαιρετικό. Προεπιλογή false. **/
    optedInNotifications?: boolean;
    /** Προαιρετικό. Προεπιλογή false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Προαιρετικό. Μέγιστο 100 χαρακτήρες. Αυτή η ετικέτα θα εμφανίζεται δίπλα στο όνομά τους. Προεπιλογή είναι Administrator/Moderator όταν ισχύει. **/
    displayLabel?: string;
    /** Προαιρετικό. Μέγιστο 500 χαρακτήρες. Αυτό θα εμφανίζεται αντί του username. **/
    displayName?: string;
    /** Προαιρετικό. Μέγιστο 2k χαρακτήρες. Το όνομα του χρήστη θα συνδέεται σε αυτό. **/
    websiteUrl?: string;
    /** Προαιρετικό. Μέχρι 100 groups ανά χρήστη. Το id μιας ομάδας δεν μπορεί να έχει μήκος μεγαλύτερο από 50 χαρακτήρες. **/
    groupIds?: string[];
    /** Προαιρετικό. Δηλώνει τον χρήστη ως διαχειριστή. **/
    isAdmin?: boolean;
    /** Προαιρετικό. Δηλώνει τον χρήστη ως moderator. **/
    isModerator?: boolean;
    /** Προαιρετικό, προεπιλογή true. Ορίστε σε false για να ενεργοποιήσετε την καρτέλα "activity" στο προφίλ του χρήστη. **/
    isProfileActivityPrivate?: boolean;
    /** Προαιρετικό, προεπιλογή false. Ορίστε σε true για να απενεργοποιήσετε τα σχόλια προφίλ. **/
    isProfileCommentsPrivate?: boolean;
    /** Προαιρετικό, προεπιλογή false. Ορίστε σε true για να απενεργοποιήσετε την απευθείας αποστολή μηνυμάτων σε αυτόν τον χρήστη. **/
    isProfileDMDisabled?: boolean;
}
[inline-code-end]

#### Moderators and Administrators

Για admins και moderators, περάστε τα αντίστοιχα flags `isAdmin` ή `isModerator` στο αντικείμενο `SSOUser`.

#### Notifications

Για να ενεργοποιήσετε ή να απενεργοποιήσετε τις ειδοποιήσεις, ορίστε την τιμή του `optedInNotifications` σε `true` ή `false` αντίστοιχα. Την πρώτη φορά που ο χρήστης φορτώνει τη σελίδα με αυτή την τιμή στο SSO payload, οι ρυθμίσεις ειδοποιήσεων του θα ενημερωθούν.

Επιπλέον, αν θέλετε οι χρήστες να λαμβάνουν ειδοποιήσεις μέσω email για δραστηριότητα στις σελίδες στις οποίες έχουν εγγραφεί (αντί μόνο ειδοποιήσεων εντός της εφαρμογής), τότε ορίστε το `optedInSubscriptionNotifications` σε `true`.

#### VIP Users & Special Labels

Μπορείτε να εμφανίσετε μια ειδική ετικέτα δίπλα στο όνομα του χρήστη χρησιμοποιώντας το προαιρετικό πεδίο "displayLabel".

#### Unauthenticated users

Για να αναπαραστήσετε έναν μη αυθεντικοποιημένο χρήστη, απλώς μην συμπληρώσετε τα userDataJSONBase64, verificationHash, ή timestamp. Παρέχετε ένα loginURL.

Αυτοί οι χρήστες δεν θα μπορούν να σχολιάσουν, και αντ' αυτού θα τους παρουσιαστεί ένα μήνυμα σύνδεσης (μήνυμα, σύνδεσμος, ή κουμπί, ανάλογα με τη διαμόρφωση).

#### Direct Examples for Serializing and Hashing User Data

Περισσότερες λεπτομέρειες και παραδείγματα <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">εδώ</a> (js), <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">εδώ</a> (java) και <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">εδώ</a> (php).

Κατανοούμε ότι οποιαδήποτε ενσωμάτωση μπορεί να είναι μια πολύπλοκη και επίπονη διαδικασία. Μη διστάσετε να επικοινωνήσετε με τον αντιπρόσωπό σας ή να χρησιμοποιήσετε τη <a href="https://fastcomments.com/auth/my-account/help" target="_blank">σελίδα υποστήριξης</a>.

---