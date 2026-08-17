[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

Το FastComments Secure SSO χρησιμοποιεί κρυπτογράφηση HMAC‑SHA256 ως μηχανισμό για την υλοποίηση του SSO. Αρχικά θα παρουσιάσουμε τη συνολική αρχιτεκτονική, θα δώσουμε παραδείγματα και λεπτομερή βήματα.

Υπάρχει επίσης τεκμηρίωση σχετικά με τη μετάβαση από άλλους παρόχους με παρόμοιους μηχανισμούς SSO και τις διαφορές.

Η ροή φαίνεται ως εξής:

<div class="screenshot white-bg">
    <div class="title">Ασφαλής Ροή SSO</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Διάγραμμα Ασφαλούς SSO" />
</div>

Δεδομένου ότι το Secure SSO περιλαμβάνει ανάπτυξη full‑stack, πλήρη παραδείγματα κώδικα σε Java/Spring, NodeJS/Express και vanilla PHP είναι επί του παρόντος διαθέσιμα στο GitHub.

Παρόλο που χρησιμοποιούμε ExpressJS στο παράδειγμα NodeJS και Spring στο παράδειγμα Java, δεν απαιτούνται frameworks/βιβλιοθήκες σε αυτά τα περιβάλλοντα για την υλοποίηση του FastComments SSO – τα ενσωματωμένα πακέτα κρυπτογράφησης λειτουργούν.

Δεν χρειάζεται να γράψετε νέα API endpoints με το FastComments SSO. Απλώς κρυπτογραφήστε τις πληροφορίες του χρήστη χρησιμοποιώντας το μυστικό κλειδί σας και περάστε το payload στο widget σχολίων.

#### Get Your API Secret Key

Το μυστικό κλειδί API σας μπορεί να ληφθεί από αυτή τη σελίδα. Μπορείτε επίσης να βρείτε αυτή τη σελίδα πηγαίνοντας στο «Ο λογαριασμός μου», κάνοντας κλικ στο πλακίδιο API/SSO και, στη συνέχεια, κάνοντας κλικ στο «Λήψη Μυστικού Κλειδιού API».

#### Comment Widget Parameters

Τεκμηρίωση API υψηλού επιπέδου για το widget σχολίων μπορεί να βρεθεί [here](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1).

Ας εμβαθύνουμε περισσότερο στο τι σημαίνουν αυτές οι παράμετροι.

Το widget σχολίων δέχεται ένα αντικείμενο ρυθμίσεων – το περνάτε ήδη εάν χρησιμοποιείτε το FastComments για να περάσετε το αναγνωριστικό πελάτη σας (ονομάζεται tenantId).

Για να ενεργοποιήσετε το SSO, περάστε ένα νέο αντικείμενο "sso", το οποίο πρέπει να περιέχει τις παρακάτω παραμέτρους. Οι τιμές πρέπει να δημιουργούνται από τον διακομιστή.

- userDataJSONBase64: Τα δεδομένα του χρήστη σε μορφή JSON, τα οποία κωδικοποιούνται στη συνέχεια σε Base64.
- verificationHash: Το hash HMAC‑SHA256 που δημιουργείται από UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Χρονική σήμανση Epoch, σε **χιλιοστά του δευτερολέπτου**. Δεν πρέπει να είναι στο μέλλον ή περισσότερο από δύο ημέρες στο παρελθόν.
- loginURL: Ένα URL που το widget σχολίων μπορεί να εμφανίσει για να συνδέσει τον χρήστη.
- logoutURL: Ένα URL που το widget σχολίων μπορεί να εμφανίσει για να αποσυνδέσει τον χρήστη.
- loginCallback: Όταν παρέχεται αντί του login URL, μια συνάρτηση που το widget σχολίων θα καλέσει όταν ο χρήστης κάνει κλικ στο κουμπί σύνδεσης.
- logoutCallback: Όταν παρέχεται αντί του logout URL, μια συνάρτηση που το widget σχολίων θα καλέσει όταν ο χρήστης κάνει κλικ στο κουμπί αποσύνδεσης.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Κώδικας Πελάτη Secure SSO'; isFunctional = false; code-example-end]

#### The User Object

[inline-code-attrs-start title = 'Το Αντικείμενο Χρήστη'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Required. 1k Characters Max. **/
    id: string;
    /** Required. 1k Characters Max. Note: Must be unique. **/
    email: string;
    /** Required. 1k Characters Max. Note: The username cannot be an email. Does not have to be unique. **/
    username: string;
    /** Optional. 3k Characters Max for URLs. Default is from gravatar based on email. Supports 64 encoded images, in which case the limit is 50k characters. **/ 
    avatar?: string;
    /** Optional. Default false. **/
    optedInNotifications?: boolean;
    /** Optional. Default false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Optional. 100 Characters Max. This label will be shown next to their name. Default is Administrator/Moderator when applicable. **/
    displayLabel?: string;
    /** Optional. 500 Characters Max. This will be shown instead of the username. **/
    displayName?: string;
    /** Optional. 2k Characters Max. The user's name will link to this. **/
    websiteUrl?: string;
    /** Optional. Up to 100 groups per user. A group id may not be longer than 50 characters. **/
    groupIds?: string[];
    /** Optional. Denotes the user as an administrator. **/
    isAdmin?: boolean;
    /** Optional. Denotes the user as a moderator. **/
    isModerator?: boolean;
    /** Optional, default true. Set to false to enable the "activity" tab in the user's profile. **/
    isProfileActivityPrivate?: boolean;
    /** Optional, default false. Set to true to disable profile comments. **/
    isProfileCommentsPrivate?: boolean;
    /** Optional, default false. Set to true to disable direct messaging this user. **/
    isProfileDMDisabled?: boolean;
    /** Optional configuration for user badges. **/
    badgeConfig?: {
        /** Array of global badge IDs to assign. Limited to 30 badges. Order is respected. **/
        badgeIds: string[];
        /** Array of badge IDs scoped to the current page (urlId). Only displayed on the assigned page. **/
        pageBadgeIds?: string[];
        /** If true, replaces existing displayed badges. Global and page-scoped are overridden independently. **/
        override?: boolean;
        /** If true, updates badge display properties from tenant configuration. **/
        update?: boolean;
    };
}
[inline-code-end]

#### Moderators and Administrators

Για διαχειριστές και συντονιστές, περάστε τις αντίστοιχες σημαίες `isAdmin` ή `isModerator` στο αντικείμενο `SSOUser`.

#### Notifications

Για να ενεργοποιήσετε ή να απενεργοποιήσετε τις ειδοποιήσεις, ορίστε την τιμή του `optedInNotifications` σε `true` ή `false` αντίστοιχα. Την πρώτη φορά που ο χρήστης φορτώνει τη σελίδα με αυτήν την τιμή στο payload SSO, οι ρυθμίσεις ειδοποιήσεων του θα ενημερωθούν.

Επιπλέον, εάν θέλετε οι χρήστες να λαμβάνουν email ειδοποιήσεις για δραστηριότητα σε σελίδες στις οποίες έχουν εγγραφεί (αντί για απλές ειδοποιήσεις εντός της εφαρμογής), ορίστε το `optedInSubscriptionNotifications` σε `true`.

#### VIP Users & Special Labels

Μπορείτε να εμφανίσετε μια ειδική ετικέτα δίπλα στο όνομα του χρήστη χρησιμοποιώντας το προαιρετικό πεδίο "displayLabel".

#### Unauthenticated users

Για να αντιπροσωπεύσετε έναν μη αυθεντικοποιημένο χρήστη, απλώς μην συμπληρώσετε τα userDataJSONBase64, verificationHash ή timestamp. Παρέχετε ένα loginURL.

Αυτοί οι χρήστες δεν θα μπορούν να σχολιάσουν και θα εμφανιστεί αντί αυτού ένα μήνυμα σύνδεσης (μήνυμα, σύνδεσμος ή κουμπί, ανάλογα με τη ρύθμιση).

#### Direct Examples for Serializing and Hashing User Data

Περισσότερες λεπτομέρειες ως παραδείγματα εδώ (js), εδώ (java) και εδώ (php).

Καταλαβαίνουμε ότι κάθε ενσωμάτωση μπορεί να είναι μια πολύπλοκη και επίπονη διαδικασία. Μη διστάσετε να επικοινωνήσετε με τον αντιπρόσωπό σας ή να χρησιμοποιήσετε τη σελίδα υποστήριξης.