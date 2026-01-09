U odeljku **Footer** na kartici Custom Code, nalepite sledeći kod:

[inline-code-attrs-start title = 'Typeflo.io Isječak koda za komentare uživo'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
    <img class="screenshot-image" src="/images/installation-guides/typeflo-step-3-paste-code-in-footer-section.png" alt="Nalepite FastComments kod u odeljak Footer" />
</div>

Nakon što nalepite kod, kliknite dugme **Sačuvaj** da primenite izmene.

Napomena: Ovaj kod sadrži logiku za dinamičko postavljanje FastComments widgeta u optimalnu poziciju na vašim Typeflo.io blog objavama. Ostali isječci koda neće ispravno raditi sa rasporedom Typeflo.io.

Zapamtite da zamenite 'demo' stvarnim FastComments tenant ID-om nakon registracije. Ako ste prijavljeni na FastComments.com, on bi već trebalo da bude zamenjen.