In the **Footer** section of the Custom Code tab, paste the following code:

[inline-code-attrs-start title = 'Typeflo.io Live коментари исечак'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
    <div class="title">Налепите код у одељку подножја</div>
    <img class="screenshot-image" src="/images/installation-guides/typeflo-step-3-paste-code-in-footer-section.png" alt="Налепите FastComments код у одељку подножја" />
</div>

After pasting the code, click the **Save** button to apply your changes.

**Напомена:** Овај код садржи логику за динамичко постављање FastComments виџета на оптимално место у вашим Typeflo.io блог постовима. Други кодни исечци неће исправно радити са распоредом Typeflo.io‑а.

Запамтите да замените `'demo'` вашим стварним FastComments tenant ID‑јем након регистрације. Ако сте пријављени на FastComments.com, он би већ требао бити замењен.