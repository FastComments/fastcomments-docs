Plak de volgende code in de **Voettekst**-sectie van het tabblad Aangepaste code:

[inline-code-attrs-start title = 'Typeflo.io Live Commentaar Codefragment'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
    <div class="title">Plak code in de Voettekst-sectie</div>
    <img class="screenshot-image" src="/images/installation-guides/typeflo-step-3-paste-code-in-footer-section.png" alt="Plak FastComments-code in de voettekstsectie" />
</div>

Klik na het plakken van de code op de knop **Opslaan** om je wijzigingen toe te passen.

Opmerking: Deze code bevat logica om de FastComments-widget dynamisch in de optimale positie op je Typeflo.io-blogposts te plaatsen. Andere codefragmenten werken niet correct met de lay-out van Typeflo.io.

Vergeet niet 'demo' te vervangen door je eigen FastComments tenant-ID nadat je je hebt aangemeld. Als je ingelogd bent op FastComments.com, zou deze al vervangen moeten zijn.