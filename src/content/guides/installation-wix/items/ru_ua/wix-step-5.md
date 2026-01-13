---
Далее настроим систему так, чтобы тред комментариев менялся в зависимости от текущей страницы, позволяя пользователям обсуждать отображаемое содержимое.

Без следующих шагов у вас будет только один глобальный тред комментариев для всего сайта — что не очень удобно.

#### Режим разработчика

Чтобы добавить эту функциональность, нам нужно перейти в то, что Wix называет `Dev Mode`.

Нажмите опцию `Dev Mode` в верхней части экрана.

<div class="screenshot white-bg">
    <div class="title">Включить Dev Mode</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-dev-mode.png" alt="Включить Dev Mode" />
</div>

#### Задайте ID элемента

Мы собираемся добавить пользовательский код для этого, но сначала нужно присвоить новому встраиваемому элементу ID, чтобы на него можно было ссылаться.

Давайте назовём его `fastcomments`.

Кликните новый встраиваемый элемент, который мы добавили, и в режиме разработчика в правом нижнем углу вы должны увидеть поле ID со значением вроде `html1`:

<div class="screenshot white-bg">
    <div class="title">Поле ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-0.png" alt="Поле ID" />
</div>

Измените его на `fastcomments` и нажмите Enter:

<div class="screenshot white-bg">
    <div class="title">Установить ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-1.png" alt="Установить ID" />
</div>

Теперь мы можем добавить пользовательский код, который сообщает области комментариев, какую страницу мы просматриваем.

Внизу экрана вы увидите редактор кода, похожий на этот:

<div class="screenshot white-bg">
    <div class="title">Открыть редактор</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-open-editor.png" alt="Открыть редактор" />
</div>

Скопируйте следующий код и вставьте его туда:

[inline-code-attrs-start title = 'Фрагмент навигации комментариев Wix'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import wixLocation from 'wix-location';

$w.onReady(function () {
    function updateFastCommentsLocation() {
        try {
            const url = (wixLocation.baseUrl + '/' + wixLocation.path).replace(/,/g, '/');
            $w('#fastcomments').postMessage({'action': 'reload', 'url': url});
        } catch (err) {
            console.error('Wix -> FastComments Error', err);
        }
    }

    updateFastCommentsLocation();
    wixLocation.onChange( () => {
        updateFastCommentsLocation();
    });
});
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">Добавить код навигации</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-paste-code.png" alt="Добавить код навигации" />
</div>

---