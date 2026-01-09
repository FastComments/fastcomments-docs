У одељку **Подножје** на картици Прилагођени код, налепите следећи код:

[inline-code-attrs-start title = 'Typeflo.io исјечак кода за коментаре уживо'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
    <div class="title">Налепите код у одељак Подножје</div>
    <img class="screenshot-image" src="/images/installation-guides/typeflo-step-3-paste-code-in-footer-section.png" alt="Налепите FastComments код у одељак Подножје" />
</div>

Након што налепите код, кликните на дугме **Сачувај** да примените измене.

Напомена: Овај код садржи логiku која динамички поставља FastComments видгет на оптималну локацију у вашим Typeflo.io блог објавама. Остали исјечци кода неће правилно радити са распоредом Typeflo.io.

Запамтите да замените `'demo'` са вашим стварним FastComments tenant ID након регистрације. Ако сте пријављени на FastComments.com, он би требало да је већ замењен.