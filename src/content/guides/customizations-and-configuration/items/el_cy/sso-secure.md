[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

Το FastComments Secure SSO χρησιμοποιεί κρυπτογράφηση HMAC-SHA256 ως μηχανισμό για την υλοποίηση SSO. Αρχικά θα παρουσιάσουμε τη συνολική αρχιτεκτονική, θα δώσουμε παραδείγματα και λεπτομερή βήματα.

Υπάρχει επίσης τεκμηρίωση σχετικά με τη μετανάστευση από άλλους παρόχους με παρόμοιους μηχανισμούς SSO, καθώς και τις διαφορές.

Η ροή φαίνεται ως εξής:

<div class="screenshot white-bg">
    <div class="title">Ροή Ασφαλούς SSO</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Διάγραμμα Ασφαλούς SSO" />
</div>

Εφόσον το Secure SSO αφορά full-stack ανάπτυξη, πλήρη λειτουργικά παραδείγματα κώδικα σε Java/Spring, NodeJS/Express και vanilla PHP βρίσκονται αυτή τη στιγμή <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">στο GitHub</a>.

Αν και χρησιμοποιούμε ExpressJS στο παράδειγμα NodeJS και Spring στο παράδειγμα Java, δεν απαιτούνται πλαίσια/βιβλιοθήκες σε αυτά τα run-times για να υλοποιήσετε το FastComments SSO - οι εγγενείς βιβλιοθήκες crypto λειτουργούν.

Δεν χρειάζεται να γράψετε νέα endpoints API με το FastComments SSO. Απλώς κρυπτογραφήστε τις πληροφορίες του χρήστη χρησιμοποιώντας το μυστικό κλειδί σας και περάστε το payload στο widget σχολίων.

#### Λάβετε το API Secret Key σας

Το API Secret σας μπορεί να ληφθεί από <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">αυτή τη σελίδα</a>. Μπορείτε επίσης να βρείτε αυτή τη σελίδα πηγαίνοντας στο Ο λογαριασμός μου, κάνοντας κλικ στο πλακίδιο API/SSO και στη συνέχεια κάνοντας κλικ στο "Λήψη Μυστικού Κλειδιού API".

#### Παράμετροι του Widget Σχολίων

Η υψηλού επιπέδου τεκμηρίωση API για το widget σχολίων βρίσκεται <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">εδώ</a>.

Ας δούμε πιο αναλυτικά τι σημαίνουν αυτές οι παράμετροι.

Το widget σχολίων παίρνει ένα αντικείμενο ρύθμισης - ήδη το περνάτε εάν χρησιμοποιείτε το FastComments για να περάσετε το tenantId σας.

Για να ενεργοποιήσετε το SSO, περάστε ένα νέο αντικείμενο "sso", το οποίο πρέπει να έχει τις ακόλουθες παραμέτρους. Οι τιμές θα πρέπει να δημιουργούνται στην πλευρά του server.

- userDataJSONBase64: Τα δεδομένα του χρήστη σε μορφή JSON, τα οποία στη συνέχεια κωδικοποιούνται σε Base64.
- verificationHash: Το HMAC-SHA256 hash που δημιουργείται από το UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Χρονική σήμανση epoch, σε **χιλιοστά του δευτερολέπτου**. Δεν πρέπει να είναι στο μέλλον ή να είναι παλαιότερη περισσότερο από δύο ημέρες.
- loginURL: Ένα URL που το widget σχολίων μπορεί να εμφανίσει για να συνδέσει τον χρήστη.
- logoutURL: Ένα URL που το widget σχολίων μπορεί να εμφανίσει για να αποσυνδέσει τον χρήστη.
- loginCallback: Όταν παρέχεται αντί του login URL, μια συνάρτηση που το widget σχολίων θα καλεί όταν γίνει κλικ στο κουμπί σύνδεσης.
- logoutCallback: Όταν παρέχεται αντί του logout URL, μια συνάρτηση που το widget σχολίων θα καλεί όταν γίνει κλικ στο κουμπί αποσύνδεσης.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### Το Αντικείμενο Χρήστη

Το αντικείμενο χρήστη περιέχει το ακόλουθο σχήμα:
[inline-code-attrs-start title = 'Το Αντικείμενο Χρήστη'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Υποχρεωτικό. Μέγιστο 1k χαρακτήρες. **/
    id: string;
    /** Υποχρεωτικό. Μέγιστο 1k χαρακτήρες. Σημείωση: Πρέπει να είναι μοναδικό. **/
    email: string;
    /** Υποχρεωτικό. Μέγιστο 1k χαρακτήρες. Σημείωση: Το username δεν μπορεί να είναι email. Δεν χρειάζεται να είναι μοναδικό. **/
    username: string;
    /** Προαιρετικό. Μέγιστο 3k χαρακτήρες για URLs. Προεπιλογή είναι από gravatar βασισμένο στο email. Υποστηρίζει εικόνες κωδικοποιημένες σε base64, οπότε σε αυτή την περίπτωση το όριο είναι 50k χαρακτήρες. **/ 
    avatar?: string;
    /** Προαιρετικό. Προεπιλογή false. **/
    optedInNotifications?: boolean;
    /** Προαιρετικό. Προεπιλογή false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Προαιρετικό. Μέγιστο 100 χαρακτήρες. Αυτή η ετικέτα θα εμφανίζεται δίπλα στο όνομά τους. Προεπιλογή είναι Administrator/Moderator όταν ισχύει. **/
    displayLabel?: string;
    /** Προαιρετικό. Μέγιστο 500 χαρακτήρες. Αυτό θα εμφανίζεται αντί του username. **/
    displayName?: string;
    /** Προαιρετικό. Μέγιστο 2k χαρακτήρες. Το όνομα του χρήστη θα συνδέεται με αυτό. **/
    websiteUrl?: string;
    /** Προαιρετικό. Έως 100 ομάδες ανά χρήστη. Το id μιας ομάδας δεν μπορεί να έχει περισσότερους από 50 χαρακτήρες. **/
    groupIds?: string[];
    /** Προαιρετικό. Δηλώνει ότι ο χρήστης είναι διαχειριστής. **/
    isAdmin?: boolean;
    /** Προαιρετικό. Δηλώνει ότι ο χρήστης είναι moderator. **/
    isModerator?: boolean;
    /** Προαιρετικό, προεπιλογή true. Ορίστε σε false για να ενεργοποιήσετε την καρτέλα "activity" στο προφίλ του χρήστη. **/
    isProfileActivityPrivate?: boolean;
    /** Προαιρετικό, προεπιλογή false. Ορίστε σε true για να απενεργοποιήσετε τα σχόλια προφίλ. **/
    isProfileCommentsPrivate?: boolean;
    /** Προαιρετικό, προεπιλογή false. Ορίστε σε true για να απενεργοποιήσετε την άμεση αποστολή μηνύματος (DM) σε αυτόν τον χρήστη. **/
    isProfileDMDisabled?: boolean;
    /** Προαιρετική ρύθμιση για τα badges του χρήστη. **/
    badgeConfig?: {
        /** Πίνακας με παγκόσμια badge IDs για ανάθεση. Περιορισμένο σε 30 badges. Η σειρά τηρείται. **/
        badgeIds: string[];
        /** Πίνακας badge IDs περιορισμένων στην τρέχουσα σελίδα (urlId). Εμφανίζονται μόνο στη σελίδα που έχει ανατεθεί. **/
        pageBadgeIds?: string[];
        /** Αν true, αντικαθιστά τα υπάρχοντα εμφανιζόμενα badges. Τα παγκόσμια και τα page-scoped αντικαθίστανται ανεξάρτητα. **/
        override?: boolean;
        /** Αν true, ενημερώνει τις ιδιότητες εμφάνισης των badges από τη ρύθμιση του tenant. **/
        update?: boolean;
    };
}
[inline-code-end]

#### Moderators and Administrators

Για admins και moderators, περάστε τα αντίστοιχα flags `isAdmin` ή `isModerator` στο αντικείμενο `SSOUser`.

#### Notifications

Για να ενεργοποιήσετε ή να απενεργοποιήσετε τις ειδοποιήσεις, ορίστε την τιμή του `optedInNotifications` σε `true` ή `false` αντίστοιχα. Την πρώτη φορά που ο χρήστης θα φορτώσει τη σελίδα με αυτή την τιμή στο SSO payload, οι ρυθμίσεις ειδοποιήσεων του θα ενημερωθούν.

Επιπλέον, αν θέλετε οι χρήστες να λαμβάνουν ειδοποιητικά emails για δραστηριότητα σε σελίδες στις οποίες έχουν εγγραφεί (αντί μόνο ειδοποιήσεων εντός εφαρμογής), τότε ορίστε το `optedInSubscriptionNotifications` σε `true`.

#### VIP Users & Special Labels

Μπορείτε να εμφανίσετε μια ειδική ετικέτα δίπλα στο όνομα του χρήστη χρησιμοποιώντας το προαιρετικό πεδίο "displayLabel".

#### Unauthenticated users

Για να αντιπροσωπεύσετε έναν μη πιστοποιημένο χρήστη, απλώς μην γεμίζετε τα userDataJSONBase64, verificationHash ή timestamp. Παρέχετε ένα loginURL.

Αυτοί οι χρήστες δεν θα μπορούν να σχολιάσουν και αντί αυτών θα τους εμφανιστεί ένα μήνυμα σύνδεσης (μήνυμα, σύνδεσμος ή κουμπί, ανάλογα με τη ρύθμιση).

#### Direct Examples for Serializing and Hashing User Data

Περισσότερες λεπτομέρειες και παραδείγματα <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">εδώ</a> (js), <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">εδώ</a> (java) και <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">εδώ</a> (php).

Κατανοούμε πως οποιαδήποτε ενσωμάτωση μπορεί να είναι μια σύνθετη και επίπονη διαδικασία. Μην διστάσετε να επικοινωνήσετε με τον εκπρόσωπό σας ή να χρησιμοποιήσετε τη <a href="https://fastcomments.com/auth/my-account/help" target="_blank">σελίδα υποστήριξης</a>.

---