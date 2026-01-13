Далее настроим всё так, чтобы поток комментариев менялся в зависимости от текущей страницы, позволяя пользователям обсуждать отображаемое сейчас содержимое.

Без следующих шагов у вас будет только один глобальный поток комментариев для всего сайта — что не очень удобно.

#### Dev Mode

Чтобы добавить эту функциональность, нам нужно войти в то, что Wix называет `Dev Mode`.

Нажмите опцию `Dev Mode` в верхней части экрана.

<div class="screenshot white-bg">
    <div class="title">Включить Dev Mode</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-dev-mode.png" alt="Включить Dev Mode" />
</div>

#### Set the Element ID

Мы собираемся добавить пользовательский код для этого, но сначала нужно присвоить новому элементу embed идентификатор, чтобы ссылаться на него.

Назовём его `fastcomments`.

Кликните добавленный элемент embed, и в режиме разработчика в правом нижнем углу вы увидите поле ID со значением вроде `html1`:

<div class="screenshot white-bg">
    <div class="title">Поле ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-0.png" alt="Поле ID" />
</div>

Измените это на `fastcomments` и нажмите Enter:

<div class="screenshot white-bg">
    <div class="title">Задать ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-1.png" alt="Задать ID" />
</div>

Теперь мы можем добавить наш пользовательский код, который сообщает области комментариев, какую страницу мы просматриваем.

Внизу экрана вы должны увидеть редактор кода, похожий на этот:

<div class="screenshot white-bg">
    <div class="title">Открыть редактор</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-open-editor.png" alt="Открыть редактор" />
</div>

Скопируйте приведённый ниже код и вставьте его туда:

[inline-code-attrs-start title = 'Сниппет навигации комментариев Wix'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
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
    <div class="title">Добавьте код навигации</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-paste-code.png" alt="Добавьте код навигации" />
</div>

---