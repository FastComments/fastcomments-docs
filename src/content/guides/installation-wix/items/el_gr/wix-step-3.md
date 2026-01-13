Αυτό το παράδειγμα χρησιμοποιεί προσαρμοσμένο κώδικα ώστε να είναι συμβατό με το Wix. **Δεν θα μπορείτε να χρησιμοποιήσετε τα αποσπάσματα κώδικα του FastComments από άλλα σεμινάρια.**

Open the form to add our custom HTML dialog by clicking `Enter Code` and selecting `HTML`:

<div class="screenshot white-bg">
    <div class="title">Βήμα 3: Άνοιγμα διαλόγου HTML</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-open-form.png" alt="Βήμα 3: Άνοιγμα διαλόγου HTML" />
</div>

Copy the following HTML snippet and paste it into the dialog, and click Ενημέρωση:

[inline-code-attrs-start title = 'Απόσπασμα κώδικα σχολίων Wix'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

<div class="screenshot white-bg">
    <div class="title">Βήμα 3: Επικόλληση και Αποθήκευση</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-paste-and-save.png" alt="Βήμα 3: Επικόλληση και Αποθήκευση" />
</div>

You should now see a very tiny version of the comment widget:

<div class="screenshot white-bg">
    <div class="title">Βήμα 3: Το Αποτέλεσμα</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-result.png" alt="Βήμα 3: Το Αποτέλεσμα" />
</div>

Στη συνέχεια θα το τοποθετήσουμε και θα προσαρμόσουμε το μέγεθός του ώστε να ταιριάζει στη σελίδα μας.