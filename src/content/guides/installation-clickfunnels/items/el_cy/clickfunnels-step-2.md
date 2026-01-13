Τώρα που βρισκόμαστε στον επεξεργαστή προτύπων, πρέπει να αποφασίσουμε πού θέλουμε να εμφανίζονται τα σχόλια ή η ζωντανή συνομιλία.

Σε αυτό το παράδειγμα θα το προσθέσουμε απευθείας κάτω από το βίντεο. Φέρτε το δείκτη πάνω από το στοιχείο όπου θέλετε να προσθέσετε το widget στο τέλος, και κάνετε κλικ στο `ADD ELEMENT`:

<div class="screenshot white-bg">
    <div class="title">Προσθήκη στοιχείου</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-4-add-element.png" alt="Προσθήκη στοιχείου" />
</div>

Επιλέξτε `CUSTOM JS/HTML`:

<div class="screenshot white-bg">
    <div class="title">Επιλέξτε CUSTOM JS/HTML</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-5-custom-js-html.png" alt="Επιλέξτε CUSTOM JS/HTML" />
</div>

Τώρα ας ανοίξουμε τον επεξεργαστή κώδικα όπου θα επικολλήσουμε τον κώδικά μας.

Το ClickFunnels είναι λίγο συγκεχυμένο σε αυτό το επόμενο βήμα.

Είναι σημαντικό να *ΜΗΝ* επιλέξετε `Code` όταν φέρετε το δείκτη πάνω από το νέο στοιχείο. Αντί γι' αυτό, επιλέξτε `SETTINGS`:

<div class="screenshot white-bg">
    <div class="title">Επιλέξτε ΡΥΘΜΙΣΕΙΣ</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-6-settings.png" alt="Επιλέξτε ΡΥΘΜΙΣΕΙΣ" />
</div>

Τώρα στη δεξιά πλευρά μπορούμε να κάνουμε κλικ στο `Open Code Editor`:

<div class="screenshot white-bg">
    <div class="title">Κάντε κλικ στο Open Code Editor</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-7-open-code-editor.png" alt="Κάντε κλικ στο Open Code Editor" />
</div>

Θα δείτε να ανοίγει ένα μεγάλο τετράγωνο. Εδώ είναι που μπορούμε να επικολλήσουμε τον κώδικά μας. Αντιγράψτε το ακόλουθο απόσπασμα (χρησιμοποιήστε το κουμπί αντιγραφής πάνω δεξιά):

[inline-code-attrs-start title = 'Απόσπασμα κώδικα ClickFunnels Streaming Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-live-chat.min.js"></script>
<div id="fastcomments-live-chat-widget" style="width: 500px;min-height: 780px;"></div>
<style>
    #fastcomments-live-chat-widget iframe {
        min-height: 780px;
    }
</style>
<script>
    (function fcLoad() {
        function tryLoad() {
            // κάποιοι πάροχοι αλλάζουν το απόσπασμα κώδικα σε async
            const container = document.getElementById('fastcomments-live-chat-widget');
            if (!container) {
                return waitRetry();
            }
            if (!window.FastCommentsLiveChat) {
                return waitRetry();
            }
            window.FastCommentsLiveChat(container, {
                tenantId: 'demo'
            });
        }
        function waitRetry() {
            setTimeout(tryLoad, 500);
        }
        tryLoad();
    })();
</script>
[inline-code-end]

Αυτό το απόσπασμα κώδικα είναι για το προϊόν μας Streaming Chat, που ταιριάζει καλά με βίντεο. Εάν θέλετε αντί για αυτό το απόσπασμα τον κώδικα του widget Live Commenting, που ταιριάζει καλύτερα με κανονικές σελίδες ή αναρτήσεις ιστολογίου, βρίσκεται στο τέλος αυτού του σεμιναρίου.

Όταν επικολλήσουμε το απόσπασμα κώδικα στο παράθυρο, θα πρέπει να φαίνεται έτσι:

<div class="screenshot white-bg">
    <div class="title">Επικόλληση Κώδικα</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-8-paste.png" alt="Επικόλληση Κώδικα" />
</div>

Τώρα απλά πρέπει να κλείσουμε το παράθυρο:

<div class="screenshot white-bg">
    <div class="title">Κλείσιμο</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-9-close.png" alt="Κλείσιμο" />
</div>

Τώρα μπορείτε να προεπισκοπήσετε τις αλλαγές σας! Μη διστάσετε να μετακινήσετε το widget και να δείτε πού σας αρέσει περισσότερο.

<div class="screenshot white-bg">
    <div class="title">Προεπισκόπηση</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-10-preview.png" alt="Προεπισκόπηση" />
</div>

Επιτυχία! Μην ξεχάσετε να δοκιμάσετε σε κινητές συσκευές!

<div class="screenshot white-bg">
    <div class="title">Επιτυχία!</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-11-success.png" alt="Επιτυχία!" />
</div>