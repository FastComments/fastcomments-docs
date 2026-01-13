Next, ας ρυθμίσουμε τα πράγματα ώστε το νήμα σχολίων να αλλάζει ανάλογα με την τρέχουσα σελίδα, επιτρέποντας στους χρήστες να συζητούν το περιεχόμενο που εμφανίζεται αυτή τη στιγμή.

Χωρίς τα παρακάτω βήματα, θα έχετε μόνο ένα παγκόσμιο νήμα σχολίων για ολόκληρο τον ιστότοπό σας - κάτι που δεν είναι ιδιαίτερα χρήσιμο.

#### Λειτουργία Dev

Για να προσθέσουμε αυτή τη λειτουργικότητα, θα πρέπει να μπούμε σε αυτό που το Wix ονομάζει `Dev Mode`.

Κάντε κλικ στην επιλογή `Dev Mode` στο πάνω μέρος της οθόνης.

<div class="screenshot white-bg">
    <div class="title">Ενεργοποίηση Dev Mode</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-dev-mode.png" alt="Ενεργοποίηση Dev Mode" />
</div>

#### Ορισμός του ID του στοιχείου

Θα προσθέσουμε προσαρμοσμένο κώδικα για να το επιτύχουμε αυτό, αλλά πρώτα πρέπει να δώσουμε στο νέο στοιχείο embed ένα ID για να αναφερόμαστε σε αυτό.

Ας το ονομάσουμε `fastcomments`.

Κάντε κλικ στο νέο στοιχείο embed που προσθέσαμε, και σε `Dev Mode`, κάτω δεξιά, θα πρέπει να δείτε ένα πεδίο ID με μια τιμή όπως `html1`:

<div class="screenshot white-bg">
    <div class="title">Το πεδίο ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-0.png" alt="Το πεδίο ID" />
</div>

Αλλάξτε αυτό σε `fastcomments` και πατήστε Enter:

<div class="screenshot white-bg">
    <div class="title">Ορισμός του ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-1.png" alt="Ορισμός του ID" />
</div>

Τώρα μπορούμε να προσθέσουμε τον προσαρμοσμένο κώδικα που λέει στην περιοχή σχολίων ποια σελίδα προβάλλουμε.

Στο κάτω μέρος της οθόνης θα πρέπει να δείτε έναν επεξεργαστή κώδικα σαν αυτόν:

<div class="screenshot white-bg">
    <div class="title">Άνοιγμα του Επεξεργαστή</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-open-editor.png" alt="Άνοιγμα του Επεξεργαστή" />
</div>

Αντιγράψτε τον ακόλουθο κώδικα και επικολλήστε τον εκεί:

[inline-code-attrs-start title = 'Απόσπασμα Πλοήγησης Σχολίων Wix'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
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
    <div class="title">Προσθήκη του Κώδικα Πλοήγησης</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-paste-code.png" alt="Προσθήκη του Κώδικα Πλοήγησης" />
</div>

---