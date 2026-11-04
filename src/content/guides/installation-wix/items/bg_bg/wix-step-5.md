След това нека настроим нещата така, че нишката с коментари да се променя според текущата страница, позволявайки на потребителите да обсъждат показваното в момента съдържание.

Без следните стъпки ще имате само една глобална нишка с коментари за целия ви сайт - което не е много полезно.

#### Dev Mode

За да добавим тази функционалност, ще трябва да влезем в това, което Wix нарича `Dev Mode`.

Кликнете върху опцията `Dev Mode` в горната част на екрана.

<div class="screenshot white-bg">
    <div class="title">Активиране на Dev Mode</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-dev-mode.png" alt="Активиране на Dev Mode" />
</div>

#### Задаване на Element ID

Ще добавим персонализиран код, за да постигнем това, но първо трябва да дадем на новия вграден елемент ID, чрез който да го препращаме.

Нека го наречем `fastcomments`.

Кликнете върху новия вграден елемент, който добавихме, и в dev mode в долния десен ъгъл трябва да видите поле ID със стойност като `html1`:

<div class="screenshot white-bg">
    <div class="title">Полето ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-0.png" alt="Полето ID" />
</div>

Променете това на `fastcomments` и натиснете enter:

<div class="screenshot white-bg">
    <div class="title">Задаване на ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-1.png" alt="Задаване на ID" />
</div>

Сега можем да добавим нашия персонализиран код, който казва на зоната с коментари коя страница разглеждаме.

В долната част на екрана трябва да видите редактор на код като този:

<div class="screenshot white-bg">
    <div class="title">Отваряне на редактора</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-open-editor.png" alt="Задаване на ID" />
</div>

Копирайте следния код и го поставете там:

[inline-code-attrs-start title = 'Wix Comments Navigation Snippet'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
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
    <div class="title">Добавяне на кода за навигация</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-paste-code.png" alt="Добавяне на кода за навигация" />
</div>