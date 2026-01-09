I **Sidefod**-sektionen på fanen **Tilpasset kode**, indsæt følgende kode:

[inline-code-attrs-start title = 'Typeflo.io Live-kommentering - kodeudsnit'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
    <div class="title">Indsæt kode i Sidefod-sektionen</div>
    <img class="screenshot-image" src="/images/installation-guides/typeflo-step-3-paste-code-in-footer-section.png" alt="Indsæt FastComments-kode i Footer-sektionen" />
</div>

Efter indsættelse af koden skal du klikke på knappen **Save** for at anvende dine ændringer.

Bemærk: Denne kode indeholder logik til dynamisk at placere FastComments-widget'en i den optimale position på dine Typeflo.io-blogindlæg. Andre kodeudsnit vil ikke fungere korrekt med Typeflo.io's layout.

Husk at erstatte 'demo' med dit faktiske FastComments tenant-id efter tilmelding. Hvis du er logget ind på FastComments.com, burde det allerede være udskiftet.