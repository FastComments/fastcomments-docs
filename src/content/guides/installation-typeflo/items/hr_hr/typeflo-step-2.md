U odjeljku **Podnožje** na kartici **Prilagođeni kod**, zalijepite sljedeći kod:

[inline-code-attrs-start title = 'Isječak koda za komentiranje uživo na Typeflo.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
    <div class="title">Zalijepite kod u odjeljak Podnožje</div>
    <img class="screenshot-image" src="/images/installation-guides/typeflo-step-3-paste-code-in-footer-section.png" alt="Zalijepite FastComments kod u odjeljak Podnožje" />
</div>

Nakon što zalijepite kod, kliknite gumb **Spremi** da biste primijenili promjene.

Napomena: Ovaj kod sadrži logiku za dinamičko postavljanje FastComments widgeta na optimalnu lokaciju na vašim Typeflo.io objavama na blogu. Drugi isječci koda neće pravilno raditi s izgledom Typeflo.io.

Zapamtite da zamijenite 'demo' sa svojim stvarnim FastComments tenant ID nakon prijave. Ako ste prijavljeni na FastComments.com, već bi trebao biti zamijenjen.