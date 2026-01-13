W sekcji **Stopka** na karcie 'Custom Code' wklej poniższy kod:

[inline-code-attrs-start title = 'Fragment kodu do komentowania na żywo Typeflo.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
    <div class="title">Wklej kod w sekcji Stopka</div>
    <img class="screenshot-image" src="/images/installation-guides/typeflo-step-3-paste-code-in-footer-section.png" alt="Wklej kod FastComments w sekcji Stopka" />
</div>

Po wklejeniu kodu kliknij przycisk **Zapisz**, aby zastosować zmiany.

Uwaga: Ten kod zawiera logikę, która dynamicznie umieszcza widget FastComments w optymalnym miejscu na wpisach bloga Typeflo.io. Inne fragmenty kodu nie będą poprawnie działać z układem Typeflo.io.

Pamiętaj, aby po rejestracji zastąpić 'demo' swoim rzeczywistym identyfikatorem tenanta FastComments. Jeśli jesteś zalogowany na FastComments.com, powinno to być już zastąpione.