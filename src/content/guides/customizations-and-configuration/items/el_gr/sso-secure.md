[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO χρησιμοποιεί κρυπτογράφηση HMAC-SHA256 ως μηχανισμό για την υλοποίηση του SSO. Πρώτα θα περιγράψουμε την συνολική αρχιτεκτονική, θα δώσουμε παραδείγματα, και λεπτομερή βήματα.

Υπάρχει επίσης κάποια τεκμηρίωση σχετικά με τη μετανάστευση από άλλους παρόχους με παρόμοιους μηχανισμούς SSO, και τις διαφορές.

Η ροή έχει ως εξής:

<div class="screenshot white-bg">
    <div class="title">Ροή Secure SSO</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Διάγραμμα Secure SSO" />
</div>

Δεδομένου ότι το Secure SSO περιλαμβάνει full-stack ανάπτυξη, πλήρη λειτουργικά παραδείγματα κώδικα σε Java/Spring, NodeJS/Express, και vanilla PHP βρίσκονται αυτή τη στιγμή <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">στο GitHub</a>.

Αν και χρησιμοποιούμε ExpressJS στο παράδειγμα NodeJS και Spring στο παράδειγμα Java, δεν απαιτούνται frameworks/libraries σε αυτά τα run-times για να υλοποιήσετε το FastComments SSO - δουλεύουν τα εγγενή πακέτα crypto.

Δεν χρειάζεται να γράψετε κανένα νέο API endpoint με το FastComments SSO. Απλώς κρυπτογραφήστε τις πληροφορίες του χρήστη χρησιμοποιώντας το secret key σας και περάστε το payload στο comment widget.

#### Get Your API Secret Key

Το API Secret σας μπορεί να ανακτηθεί από <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">αυτή τη σελίδα</a>. Μπορείτε επίσης να βρείτε αυτή τη σελίδα πηγαίνοντας στο Ο λογαριασμός μου (My Account), κάνοντας κλικ στο πλακίδιο API/SSO, και στη συνέχεια κάνοντας κλικ στο "Get API Secret Key".

#### Comment Widget Parameters

Η τεκμηρίωση API υψηλού επιπέδου για το comment widget βρίσκεται <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">εδώ</a>.

Ας δούμε πιο λεπτομερώς τι σημαίνουν αυτές οι παράμετροι.

Το comment widget λαμβάνει ένα αντικείμενο διαμόρφωσης - το περνάτε ήδη αυτό εάν χρησιμοποιείτε FastComments για να περάσετε το id του πελάτη σας (που ονομάζεται tenantId).

Για να ενεργοποιήσετε το SSO, περάστε ένα νέο αντικείμενο "sso", το οποίο πρέπει να έχει τις ακόλουθες παραμέτρους. Οι τιμές πρέπει να δημιουργούνται από το server side.

- userDataJSONBase64: Τα δεδομένα του χρήστη σε μορφή JSON, τα οποία στη συνέχεια κωδικοποιούνται σε Base64.
- verificationHash: Το hash HMAC-SHA256 που δημιουργείται από UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Χρονική σήμανση epoch, σε **milliseconds**. Δεν πρέπει να είναι στο μέλλον, ούτε περισσότερες από δύο ημέρες στο παρελθόν.
- loginURL: Ένα URL που το comment widget μπορεί να εμφανίσει για να κάνει login ο χρήστης.
- logoutURL: Ένα URL που το comment widget μπορεί να εμφανίσει για να κάνει logout ο χρήστης.
- loginCallback: Όταν παρέχεται αντί του login URL, μια συνάρτηση που το comment widget θα καλεί όταν πατηθεί το κουμπί login.
- logoutCallback: Όταν παρέχεται αντί του logout URL, μια συνάρτηση που το comment widget θα καλεί όταν πατηθεί το κουμπί logout.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### The User Object

The User object contains the following schema:
[inline-code-attrs-start title = 'Το Αντικείμενο Χρήστη'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Απαιτούμενο. 1k Characters Max. **/
    id: string;
    /** Απαιτούμενο. 1k Characters Max. Σημείωση: Πρέπει να είναι μοναδικό. **/
    email: string;
    /** Απαιτούμενο. 1k Characters Max. Σημείωση: Το username δεν μπορεί να είναι ένα email. Δεν χρειάζεται να είναι μοναδικό. **/
    username: string;
    /** Προαιρετικό. 3k Characters Max για URLs. Προεπιλογή είναι από gravatar βάσει του email. Υποστηρίζει εικόνες κωδικοποιημένες σε base64, οπότε στην περίπτωση αυτή το όριο είναι 50k χαρακτήρες. **/ 
    avatar?: string;
    /** Προαιρετικό. Προεπιλογή false. **/
    optedInNotifications?: boolean;
    /** Προαιρετικό. Προεπιλογή false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Προαιρετικό. 100 Characters Max. Αυτή η ετικέτα θα εμφανίζεται δίπλα στο όνομά τους. Προεπιλογή είναι Administrator/Moderator όταν ισχύει. **/
    displayLabel?: string;
    /** Προαιρετικό. 500 Characters Max. Αυτό θα εμφανίζεται αντί του username. **/
    displayName?: string;
    /** Προαιρετικό. 2k Characters Max. Το όνομα του χρήστη θα συνδέεται με αυτό. **/
    websiteUrl?: string;
    /** Προαιρετικό. Έως 100 groups ανά χρήστη. Ένα group id δεν μπορεί να είναι μεγαλύτερο από 50 χαρακτήρες. **/
    groupIds?: string[];
    /** Προαιρετικό. Δηλώνει τον χρήστη ως administrator. **/
    isAdmin?: boolean;
    /** Προαιρετικό. Δηλώνει τον χρήστη ως moderator. **/
    isModerator?: boolean;
    /** Προαιρετικό, προεπιλογή true. Θέστε σε false για να ενεργοποιήσετε την καρτέλα "activity" στο προφίλ του χρήστη. **/
    isProfileActivityPrivate?: boolean;
    /** Προαιρετικό, προεπιλογή false. Θέστε σε true για να απενεργοποιήσετε τα σχόλια προφίλ. **/
    isProfileCommentsPrivate?: boolean;
    /** Προαιρετικό, προεπιλογή false. Θέστε σε true για να απενεργοποιήσετε την απευθείας αποστολή μηνυμάτων σε αυτόν τον χρήστη. **/
    isProfileDMDisabled?: boolean;
    /** Προαιρετική ρύθμιση για σήματα χρήστη. **/
    badgeConfig?: {
        /** Πίνακας παγκόσμιων badge IDs προς ανάθεση. Περιορισμένο σε 30 badges. Η σειρά διατηρείται. **/
        badgeIds: string[];
        /** Πίνακας badge IDs περιορισμένων στην τρέχουσα σελίδα (urlId). Εμφανίζεται μόνο στην ανατεθειμένη σελίδα. **/
        pageBadgeIds?: string[];
        /** Αν true, αντικαθιστά τα υπάρχοντα εμφανιζόμενα badges. Τα παγκόσμια και τα σελίδα-περιορισμένα παρακάμπτονται ανεξάρτητα. **/
        override?: boolean;
        /** Αν true, ενημερώνει τις ιδιότητες εμφάνισης των badges από τη διαμόρφωση του tenant. **/
        update?: boolean;
    };
}
[inline-code-end]

#### Moderators and Administrators

Για admins και moderators, περάστε τις αντίστοιχες σημαίες `isAdmin` ή `isModerator` στο αντικείμενο `SSOUser`.

#### Notifications

Για να ενεργοποιήσετε ή να απενεργοποιήσετε τις ειδοποιήσεις, ορίστε την τιμή του `optedInNotifications` σε `true` ή `false` αντίστοιχα. Την πρώτη φορά που ο χρήστης φορτώνει τη σελίδα με αυτή την τιμή στο SSO payload, οι ρυθμίσεις ειδοποιήσεων του θα ενημερωθούν.

Επιπλέον, αν θέλετε οι χρήστες να λαμβάνουν email ειδοποιήσεων για δραστηριότητα σε σελίδες στις οποίες έχουν εγγραφεί (αντί για μόνο εντός της εφαρμογής), τότε ορίστε το `optedInSubscriptionNotifications` σε `true`.

#### VIP Users & Special Labels

Μπορείτε να εμφανίσετε μια ειδική ετικέτα δίπλα στο όνομα του χρήστη χρησιμοποιώντας το προαιρετικό πεδίο "displayLabel".

#### Unauthenticated users

Για να αναπαραστήσετε έναν μη-επαληθευμένο χρήστη, απλώς μην συμπληρώσετε τα userDataJSONBase64, verificationHash, ή timestamp. Παρέχετε ένα loginURL.

Αυτοί οι χρήστες δεν θα μπορούν να σχολιάσουν, και αντ' αυτού θα τους εμφανιστεί ένα μήνυμα σύνδεσης (μήνυμα, σύνδεσμος, ή κουμπί, ανάλογα με τη διαμόρφωση).

#### Direct Examples for Serializing and Hashing User Data

Περισσότερες λεπτομέρειες και παραδείγματα υπάρχουν <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">εδώ</a> (js), <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">εδώ</a> (java) και <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">εδώ</a> (php).

Κατανοούμε ότι οποιαδήποτε ενσωμάτωση μπορεί να είναι μια περίπλοκη και επώδυνη διαδικασία. Μην διστάσετε να επικοινωνήσετε με τον αντιπρόσωπό σας ή να χρησιμοποιήσετε τη <a href="https://fastcomments.com/auth/my-account/help" target="_blank">σελίδα υποστήριξης</a>.