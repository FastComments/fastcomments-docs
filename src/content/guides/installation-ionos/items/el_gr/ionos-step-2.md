Στη συνέχεια θα προσθέσουμε τον κώδικα του widget FastComments στον ιστότοπό σας. Αυτός ο κώδικας θα αναζητήσει όλες τις φόρμες με τον τίτλο `FastComments Goes Here` και
θα τις αντικαταστήσει με το FastComments.

Πάμε λοιπόν στο `Settings` στο κάτω αριστερό μέρος του επεξεργαστή ιστότοπου:

<div class="screenshot white-bg">
    <div class="title">Άνοιγμα Settings</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-settings.png" alt="Άνοιγμα Settings" />
</div>

Ανοίξτε την ενότητα `Custom Head Code`:

<div class="screenshot white-bg">
    <div class="title">Άνοιγμα Custom Head Code</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-custom-head-code.png" alt="Άνοιγμα Custom Head Code" />
</div>

Για το Ionos χρειαζόμαστε μια **ειδική έκδοση** του κώδικα του widget FastComments. Τα αποσπάσματα κώδικα από **άλλα οδηγίες δεν θα λειτουργήσουν.**

Τώρα αντιγράψτε τον παρακάτω κώδικα:

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
                    // we get the element that is not full width
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

...και επικολλήστε το όπως φαίνεται:

<div class="screenshot white-bg">
    <div class="title">Επικόλληση και Αποθήκευση</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-paste-and-save.png" alt="Επικόλληση και Αποθήκευση" />
</div>