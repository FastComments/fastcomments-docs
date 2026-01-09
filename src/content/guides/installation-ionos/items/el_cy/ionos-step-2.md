Next θα προσθέσουμε τον κώδικα του widget FastComments στην ιστοσελίδα σας. Αυτός ο κώδικας θα αναζητήσει όλες τις φόρμες με τίτλο `FastComments Goes Here` και θα τις αντικαταστήσει με το FastComments.

So let's go to `Settings` in the bottom left of the site editor:

<div class="screenshot white-bg">
    <div class="title">Άνοιγμα Ρυθμίσεων</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-settings.png" alt="Άνοιγμα Ρυθμίσεων" />
</div>

Open the `Custom Head Code` section:

<div class="screenshot white-bg">
    <div class="title">Άνοιγμα προσαρμοσμένου κώδικα κεφαλίδας</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-custom-head-code.png" alt="Άνοιγμα προσαρμοσμένου κώδικα κεφαλίδας" />
</div>

For Ionos we need a **special version** of the FastComments widget code. Code snippets from **other tutorials will not work.**

Now copy the following code:

[inline-code-attrs-start title = 'Απόσπασμα FastComments για Ionos'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<script>
    (function () {
        let loaded = false;
        let interval = null;

        function attemptLoad() {
            const nodes = document.querySelectorAll('h2');

            nodes.forEach(function (node) {
                if (node.innerText && node.innerText.trim() === 'FastComments Goes Here') {
                    // παίρνουμε το στοιχείο που δεν είναι πλήρους πλάτους
                    const target = node.parentNode.parentNode.parentNode.parentNode.parentNode;
                    target.innerHTML = '';
                    FastCommentsUI(target, {
                        tenantId: "demo"
                    });
                    interval && clearInterval(interval);
                    loaded = true;
                }
            });
        }

        attemptLoad();
        if (!loaded) {
            interval = setInterval(attemptLoad, 300);
        }
    })();
</script>
[inline-code-end]

...and paste it in as shown:

<div class="screenshot white-bg">
    <div class="title">Επικόλληση και Αποθήκευση</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-paste-and-save.png" alt="Επικόλληση και Αποθήκευση" />
</div>

---