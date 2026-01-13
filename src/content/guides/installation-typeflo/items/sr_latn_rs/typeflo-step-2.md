U odeljku **Footer** na kartici Custom Code, nalepite sledeći kod:

[inline-code-attrs-start title = 'Typeflo.io kod za komentarisanje uživo'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
    <div class="title">Nalepite kod u odeljak Footer</div>
    <img class="screenshot-image" src="/images/installation-guides/typeflo-step-3-paste-code-in-footer-section.png" alt="Nalepite FastComments kod u Footer odeljak" />
</div>

Nakon lepljenja koda, kliknite na dugme **Save** da primenite vaše promene.

Napomena: Ovaj kod sadrži logiku za dinamičko postavljanje FastComments vidžeta u optimalnu poziciju na vašim Typeflo.io objavama na blogu. Ostali isječci koda neće pravilno raditi sa rasporedom Typeflo.io.

Zapamtite da zamenite `'demo'` vašim stvarnim FastComments tenant ID-jem nakon registracije. Ako ste prijavljeni na FastComments.com, trebalo bi da je već zamenjen.