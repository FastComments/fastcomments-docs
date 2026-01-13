Овај пример користи прилагођени код како би био компатибилан са Wix-ом. **Нећете моћи да користите FastComments исечке кода из других туторијала.**

Отворите формулар да бисте додали наш прилагођени HTML дијалог кликом на `Enter Code` и избором `HTML`:

<div class="screenshot white-bg">
    <div class="title">Корак 3: Отворите HTML дијалог</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-open-form.png" alt="Корак 3: Отворите HTML дијалог" />
</div>

Копирајте следећи HTML исечак и налепите га у дијалог, па кликните на Ажурирај:

[inline-code-attrs-start title = 'Пример кода за коментаре на Wix'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const config = {
            tenantId: "demo"
        };
        const instance = FastCommentsUI(document.getElementById('fastcomments-widget'), config);
        window.onmessage = (event) => {
            if (event.data) {
                if (event.data.action === 'reload') {
                    console.log('Updating FastComments:', event.data.url);
                    config.urlId = event.data.url;
                    config.url = event.data.url;
                    instance.update(config);
                }
            }
        }
    })();
</script>
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">Корак 3: Налепите и сачувајте</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-paste-and-save.png" alt="Корак 3: Налепите и сачувајте" />
</div>

Сада бисте требали видети веома малу верзију виджета за коментаре:

<div class="screenshot white-bg">
    <div class="title">Корак 3: Резултат</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-result.png" alt="Корак 3: Резултат" />
</div>

Даље ћемо позиционирати и подесити величину овог елемента да одговара нашој страници.