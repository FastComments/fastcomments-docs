В разделе **Footer** вкладки Custom Code вставьте следующий код:

[inline-code-attrs-start title = 'Фрагмент кода для живых комментариев Typeflo.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
    <div class="title">Вставьте код в раздел Footer</div>
    <img class="screenshot-image" src="/images/installation-guides/typeflo-step-3-paste-code-in-footer-section.png" alt="Вставьте код FastComments в раздел Footer" />
</div>

После вставки кода нажмите кнопку **Save**, чтобы применить изменения.

Примечание: Этот код содержит логику для динамического размещения виджета FastComments в оптимальном месте на страницах вашего блога Typeflo.io. Другие фрагменты кода не будут корректно работать с версткой Typeflo.io.

Не забудьте заменить `'demo'` на ваш реальный tenant ID FastComments после регистрации. Если вы вошли в систему на FastComments.com, он должен быть уже заменён.