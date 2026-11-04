Στη συνέχεια, ας ρυθμίσουμε τα πράγματα ώστε το νήμα σχολίων να αλλάζει ανάλογα με την τρέχουσα σελίδα, επιτρέποντας στους χρήστες να συζητούν το περιεχόμενο που εμφανίζεται αυτή τη στιγμή.

Χωρίς τα παρακάτω βήματα, θα έχετε μόνο ένα καθολικό νήμα σχολίων για ολόκληρο τον ιστότοπό σας - κάτι που δεν είναι πολύ χρήσιμο.

#### Dev Mode

Για να προσθέσουμε αυτή τη λειτουργικότητα, θα πρέπει να μπούμε σε αυτό που η Wix ονομάζει `Dev Mode`.

Κάντε κλικ στην επιλογή `Dev Mode` στο πάνω μέρος της οθόνης.

<div class="screenshot white-bg">
    <div class="title">Ενεργοποίηση Dev Mode</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-dev-mode.png" alt="Ενεργοποίηση Dev Mode" />
</div>

#### Ορισμός του ID του στοιχείου

Θα προσθέσουμε προσαρμοσμένο κώδικα για να το επιτύχουμε αυτό, αλλά πρώτα πρέπει να δώσουμε στο νέο στοιχείο ενσωμάτωσης ένα ID ώστε να μπορούμε να αναφερόμαστε σε αυτό.

Ας το ονομάσουμε `fastcomments`.

Κάντε κλικ στο νέο στοιχείο ενσωμάτωσης που προσθέσαμε, και σε λειτουργία dev στο κάτω δεξί μέρος θα δείτε ένα πεδίο ID με μια τιμή όπως `html1`:

<div class="screenshot white-bg">
    <div class="title">Το πεδίο ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-0.png" alt="Το πεδίο ID" />
</div>

Αλλάξτε το σε `fastcomments` και πατήστε enter:

<div class="screenshot white-bg">
    <div class="title">Ορισμός του ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-1.png" alt="Ορισμός του ID" />
</div>

Τώρα μπορούμε να προσθέσουμε τον προσαρμοσμένο κώδικα που λέει στην περιοχή σχολίων ποια σελίδα βλέπουμε.

Στο κάτω μέρος της οθόνης θα δείτε έναν επεξεργαστή κώδικα όπως αυτόν:

<div class="screenshot white-bg">
    <div class="title">Άνοιγμα του επεξεργαστή</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-open-editor.png" alt="Ορισμός του ID" />
</div>

Αντιγράψτε τον παρακάτω κώδικα και επικολλήστε τον εκεί:

[inline-code-attrs-start title = 'Απόσπασμα πλοήγησης σχολίων Wix'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import wixLocation from 'wix-location';

$w.onReady(function () {
    function updateFastCommentsLocation() {
        try {
            const url = (wixLocation.baseUrl + '/' + wixLocation.path).replace(/,/g, '/');
            $w('#fastcomments').postMessage({'action': 'reload', 'url': url});
        } catch (err) {
            console.error('Wix -> FastComments Error', err);
        }
    }

    updateFastCommentsLocation();
    wixLocation.onChange( () => {
        updateFastCommentsLocation();
    });
});
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">Προσθήκη του κώδικα πλοήγησης</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-paste-code.png" alt="Προσθήκη του κώδικα πλοήγησης" />
</div>