У одељку **Footer** на картици Custom Code, налепите следећи код:

[inline-code-attrs-start title = 'Typeflo.io исечак кода за коментаре уживо'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
    <div class="title">Налепите код у одељак Footer</div>
    <img class="screenshot-image" src="/images/installation-guides/typeflo-step-3-paste-code-in-footer-section.png" alt="Налепите FastComments код у одељак Footer" />
</div>

После налепљивања кода, кликните на дугме **Save** да бисте применили измене.

Напомена: Овај код садржи логику која динамички поставља FastComments виџет на оптимално место у вашим чланцима на Typeflo.io блогу. Други исечци кода неће радити исправно са распоредом Typeflo.io-а.

Не заборавите да замените `'demo'` својим стварним FastComments tenant ID-јем након регистрације. Ако сте пријављени на FastComments.com, он је већ замењен.