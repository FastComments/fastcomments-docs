В секцията **Footer** на таба Custom Code поставете следния код:

[inline-code-attrs-start title = 'Фрагмент от код за живо коментиране на Typeflo.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
    <div class="title">Поставете кода в секцията Footer</div>
    <img class="screenshot-image" src="/images/installation-guides/typeflo-step-3-paste-code-in-footer-section.png" alt="Поставете кода на FastComments в секцията Footer" />
</div>

След като поставите кода, натиснете бутона **Запази**, за да приложите промените си.

Забележка: Този код включва логика за динамично поставяне на джаджата на FastComments в оптималното място в публикациите на вашия Typeflo.io блог. Други фрагменти от код няма да работят правилно с оформлението на Typeflo.io.

Не забравяйте да замените `'demo'` с вашия реален FastComments tenant ID след регистрация. Ако сте влезли в FastComments.com, той вече трябва да е заменен.