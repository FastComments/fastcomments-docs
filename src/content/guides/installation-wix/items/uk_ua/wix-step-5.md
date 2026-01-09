Далі налаштуємо так, щоб потік коментарів змінювався залежно від поточної сторінки, дозволяючи користувачам обговорювати відображений вміст.

Без наступних кроків у вас буде лише один глобальний потік коментарів для всього сайту — що не дуже корисно.

#### Режим розробника

Щоб додати цю функціональність, нам потрібно увійти в те, що Wix називає `Dev Mode`.

Натисніть опцію `Dev Mode` у верхній частині екрана.

<div class="screenshot white-bg">
    <div class="title">Увімкнути Dev Mode</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-dev-mode.png" alt="Увімкнути Dev Mode" />
</div>

#### Встановити ID елемента

Ми додаватимемо власний код для цього, але спочатку потрібно присвоїти новому вбудованому елементу ідентифікатор, за яким будемо звертатися.

Назвімо його `fastcomments`.

Клацніть по новому вбудованому елементу, який ми додали, і в режимі розробника (у нижньому правому куті) ви побачите поле ID зі значенням на кшталт `html1`:

<div class="screenshot white-bg">
    <div class="title">Поле ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-0.png" alt="Поле ID" />
</div>

Змініть його на `fastcomments` і натисніть Enter:

<div class="screenshot white-bg">
    <div class="title">Встановити ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-1.png" alt="Встановити ID" />
</div>

Тепер ми можемо додати власний код, який повідомляє області коментарів, яку сторінку ми переглядаємо.

У нижній частині екрана ви повинні побачити редактор коду, схожий на цей:

<div class="screenshot white-bg">
    <div class="title">Відкрити редактор</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-open-editor.png" alt="Відкрити редактор" />
</div>

Скопіюйте наступний код і вставте його туди:

[inline-code-attrs-start title = 'Фрагмент навігації коментарів Wix'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
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
    <div class="title">Додати код навігації</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-paste-code.png" alt="Додати код навігації" />
</div>

---