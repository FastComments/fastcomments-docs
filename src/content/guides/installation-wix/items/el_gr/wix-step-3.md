Αυτό το παράδειγμα χρησιμοποιεί προσαρμοσμένο κώδικα για συμβατότητα με το Wix. **Δεν θα μπορείτε να χρησιμοποιήσετε τα αποσπάσματα κώδικα του FastComments από άλλα σεμινάρια.**

Ανοίξτε τη φόρμα για να προσθέσουμε το προσαρμοσμένο διάλογο HTML μας κάνοντας κλικ στο `Enter Code` και επιλέγοντας `HTML`:

<div class="screenshot white-bg">
    <div class="title">Βήμα 3: Άνοιγμα Διαλόγου HTML</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-open-form.png" alt="Βήμα 3: Άνοιγμα Διαλόγου HTML" />
</div>

Αντιγράψτε το παρακάτω απόσπασμα HTML και επικολλήστε το στον διάλογο, και κάντε κλικ στο Update:

[inline-code-attrs-start title = 'Απόσπασμα Κώδικα Σχολίων Wix'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const config = {
            tenantId: "demo"
        };
        const instance = FastCommentsUI(document.getElementById('fastcomments-widget'), config);
        window.onmessage = (event) => {
            if (event.data) {
                if (event.data.action === 'reload') {
                    console.log('Updating FastComments:', event.data.url);
                    config.urlId = event.data.url;
                    config.url = event.data.url;
                    instance.update(config);
                }
            }
        }
    })();
</script>
[inline-code-end]

Τώρα θα δείτε μια πολύ μικρή έκδοση του widget σχολίων:

<div class="screenshot white-bg">
    <div class="title">Βήμα 3: Επικόλληση και Αποθήκευση</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-paste-and-save.png" alt="Βήμα 3: Επικόλληση και Αποθήκευση" />
</div>

Στη συνέχεια θα το τοποθετήσουμε και θα το μεγεθύνουμε ώστε να ταιριάζει στη σελίδα μας.

<div class="screenshot white-bg">
    <div class="title">Βήμα 3: Το Αποτέλεσμα</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-result.png" alt="Βήμα 3: Το Αποτέλεσμα" />
</div>