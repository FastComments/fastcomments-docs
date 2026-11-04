Далее настроим всё так, чтобы ветка комментариев менялась в зависимости от текущей страницы, позволяя пользователям обсуждать отображаемый контент.

Без следующих шагов у вас будет только одна глобальная ветка комментариев для всего сайта — что не очень удобно.

#### Режим разработки (Dev Mode)

Чтобы добавить эту функциональность, нам нужно перейти в так называемый `Dev Mode` в Wix.

Нажмите на опцию `Dev Mode` в верхней части экрана.

<div class="screenshot white-bg">
    <div class="title">Включите Dev Mode</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-dev-mode.png" alt="Включите Dev Mode" />
</div>

#### Задайте ID элемента

Мы добавим пользовательский код для этого, но сначала нужно задать ID новому элементу встраивания, чтобы ссылаться на него.

Назовём его `fastcomments`.

Нажмите на новый элемент встраивания, который мы добавили, и в режиме разработки в правом нижнем углу вы должны увидеть поле ID со значением вроде `html1`:

<div class="screenshot white-bg">
    <div class="title">Поле ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-0.png" alt="Поле ID" />
</div>

Измените его на `fastcomments` и нажмите Enter:

<div class="screenshot white-bg">
    <div class="title">Задайте ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-1.png" alt="Задайте ID" />
</div>

Теперь мы можем добавить пользовательский код, который сообщает области комментариев, какую страницу мы просматриваем.

В нижней части экрана вы должны увидеть редактор кода, как этот:

<div class="screenshot white-bg">
    <div class="title">Откройте редактор</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-open-editor.png" alt="Задайте ID" />
</div>

Скопируйте следующий код и вставьте его туда:

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