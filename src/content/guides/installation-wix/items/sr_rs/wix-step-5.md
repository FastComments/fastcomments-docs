Next, let's set things up so that the comment thread changes based on the current page, allowing users to discuss the currently displayed content.

Without the following steps, you will only have one global comment thread for your entire site - which is not very useful.

#### Режим за развој

Да бисмо додали ову функционалност, мораћемо да уђемо у оно што Wix назива `Dev Mode`.

Кликните на опцију `Dev Mode` на врху екрана.

<div class="screenshot white-bg">
    <div class="title">Омогућите режим за развој</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-dev-mode.png" alt="Омогућите режим за развој" />
</div>

#### Подесите ID елемента

Додаћемо прилагођени код да то остваримо, али прво треба да новом уграђеном елементу доделимо ID по коме ћемо се позивати.

Назваћемо га `fastcomments`.

Кликните на нови уграђени елемент који смо додали, и у режиму за развој у доњем десном углу требало би да видите поље ID са вредношћу попут `html1`:

<div class="screenshot white-bg">
    <div class="title">Поље ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-0.png" alt="Поље ID" />
</div>

Промените ово у `fastcomments` и притисните enter:

<div class="screenshot white-bg">
    <div class="title">Поставите ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-1.png" alt="Поставите ID" />
</div>

Сада можемо додати наш прилагођени код који ће коментарском подручју рећи коју страницу прегледавамо.

На дну екрана требало би да видите уређивач кода попут овог:

<div class="screenshot white-bg">
    <div class="title">Отворите уређивач</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-open-editor.png" alt="Поставите ID" />
</div>

Копирајте следећи код и залепите га тамо:

[inline-code-attrs-start title = 'Wix skripta za navigaciju komentara'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
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
    <div class="title">Додајте навигациони код</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-paste-code.png" alt="Додајте навигациони код" />
</div>

---