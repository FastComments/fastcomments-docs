У розділі **Footer** на вкладці Custom Code вставте наступний код:

[inline-code-attrs-start title = 'Фрагмент коду живого коментування Typeflo.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
    <div class="title">Вставте код у розділ Footer</div>
    <img class="screenshot-image" src="/images/installation-guides/typeflo-step-3-paste-code-in-footer-section.png" alt="Вставте код FastComments у розділ Footer" />
</div>

Після вставки коду натисніть кнопку **Save**, щоб застосувати зміни.

Примітка: Цей код містить логіку для динамічного розміщення віджета FastComments в оптимальному місці у ваших публікаціях блогу Typeflo.io. Інші фрагменти коду не працюватимуть належним чином із макетом Typeflo.io.

Не забудьте замінити `'demo'` на ваш фактичний ідентифікатор орендаря FastComments після реєстрації. Якщо ви увійшли на FastComments.com, він, ймовірно, вже замінений.