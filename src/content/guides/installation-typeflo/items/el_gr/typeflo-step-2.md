Στην ενότητα **Υποσέλιδο** της καρτέλας Προσαρμοσμένου Κώδικα, επικολλήστε τον ακόλουθο κώδικα:

[inline-code-attrs-start title = 'Απόσπασμα κώδικα ζωντανών σχολίων Typeflo.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js" async></script>
<script>
    (function () {
        function load() {
            let target = null;
            let lastInstance;
            if (document.querySelector('.fastcomments-widget')) {
                setTimeout(load, 1000);
                return;
            }
            if (lastInstance) {
                lastInstance.destroy();
            }
            if (window.FastCommentsUI) {
                const newElement = document.createElement('div');
                newElement.classList.add('fastcomments-widget');
                const subscribeSection = document.querySelector('.nc-SectionSubscribe2');
                if (subscribeSection) {
                    subscribeSection.parentNode.insertBefore(newElement, subscribeSection);
                    target = newElement;
                } else {
                    const fullWidthSection = document.querySelector('.container.w-full');
                    if (fullWidthSection) {
                        fullWidthSection.prepend(newElement);
                        target = newElement;
                    }
                }
            }
            if (target) {
                lastInstance = FastCommentsUI(target, {
                    "tenantId": "demo"
                });
            }
            setTimeout(load, 1000);
        }

        load();
    })();
</script>
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">Επικόλληση κώδικα στην ενότητα Υποσέλιδου</div>
    <img class="screenshot-image" src="/images/installation-guides/typeflo-step-3-paste-code-in-footer-section.png" alt="Επικόλληση κώδικα FastComments στην ενότητα υποσέλιδου" />
</div>

Μετά την επικόλληση του κώδικα, κάντε κλικ στο κουμπί **Αποθήκευση** για να εφαρμόσετε τις αλλαγές σας.

Σημείωση: Αυτός ο κώδικας περιλαμβάνει λογική για να τοποθετεί δυναμικά το widget του FastComments στην βέλτιστη θέση στις δημοσιεύσεις του Typeflo.io. Άλλα αποσπάσματα κώδικα δεν θα λειτουργήσουν σωστά με τη διάταξη του Typeflo.io.

Θυμηθείτε να αντικαταστήσετε το `'demo'` με το πραγματικό tenant ID του FastComments αφού εγγραφείτε. Εάν έχετε συνδεθεί στο FastComments.com, θα έχει ήδη αντικατασταθεί.