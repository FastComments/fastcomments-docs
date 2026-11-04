Затим, хаје да подесимо ствари тако да се нит коментара мења у зависности од тренутне странице, омогућавајући корисницима да дискутују о тренутно приказаном садржају.

Без следећих корака, имаћете само једну глобалну нит коментара за цео сајт - што није баш корисно.

#### Дев Мод

Да бисмо додали ову функционалност, мораћемо да уђемо у оно што Wix назива `Dev Mode`.

Кликните на опцију `Dev Mode` на врху екрана.

<div class="screenshot white-bg">
    <div class="title">Омогући Дев Мод</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-dev-mode.png" alt="Омогући Дев Мод" />
</div>

#### Поставите ID елемента

Додаћемо прилагођени код да бисмо то постигли, али прво морамо дати новом embed елементу ID на који ћемо се позивати.

Хајде да га назовемо `fastcomments`.

Кликните на нови embed елемент који смо додали, и у дев мод у доњем десном углу требало би да видите ID поље са вредношћу попут `html1`:

<div class="screenshot white-bg">
    <div class="title">ID Поље</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-0.png" alt="ID Поље" />
</div>

Промените ово у `fastcomments` и притисните ентер:

<div class="screenshot white-bg">
    <div class="title">Постави ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-1.png" alt="Постави ID" />
</div>

Сада можемо додати наш прилагођени код који говори области коментара коју страницу гледамо.

На дну екрана требало би да видите уређивач кода попут овог:

<div class="screenshot white-bg">
    <div class="title">Отвори Уређивач</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-open-editor.png" alt="Постави ID" />
</div>

Копирајте следећи код и налепите га тамо:

[inline-code-attrs-start title = 'Wix Исечак за навигацију коментара'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
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
    <div class="title">Додај Код за Навигацију</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-paste-code.png" alt="Додај Код за Навигацију" />
</div>