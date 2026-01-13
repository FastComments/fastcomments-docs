За Корак 2 морамо да копирамо наш исечак кода. Проверите да ли линија 50 не каже "demo" — потребно је да у њој стоји ваш tenant id. Требало би да буде попуњено за вас.

Сада да копирамо наш ThriveCart-Learn-Specific FastComments исечак кода.

Он је прилично велики, јер интеграција са ThriveCart има много функција, па само кликните на дугме Copy у горњем десном углу исечка кода:

[inline-code-attrs-start title = 'ThriveCart Learn+ Код коментара'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        let attemptsRemaining = 10;

        function tryLoad() {
            const simpleSSO = {optedInNotifications: true, optedInSubscriptionNotifications: true};
            let isAuthenticated = false;
            let profileLink = document.querySelector('.thrivecart-courses-header-profile-link');
            if (!profileLink) {
                profileLink = document.querySelector('.thrivecart-courses-header-profile'); // класа је другачија за преглед.
            }
            // широк селектор поља за е-пошту у случају да ThriveCart промијени id.
            const emailInputField = document.querySelector('input[type=email]');
            if (emailInputField && emailInputField.value) {
                isAuthenticated = true;
                simpleSSO.email = emailInputField.value;
            } else if (profileLink && !profileLink.innerText.includes('John Smith')) { // омогућити рад прегледа - нема доступне е-поште.
                attemptsRemaining--;
                if (!attemptsRemaining) {
                    return console.error('Could not load FastComments - could not determine user information (email). Please reach out to FastComments support.');
                }
                console.warn('FastComments: No user email found - waiting and trying again.');
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // повећај време чекања након 5 покушаја у случају спорог интернета.
            }
            if (profileLink) {
                // користи сурову "img" претрагу у случају да ThriveCart промени селектор класе слике.
                const avatarImg = profileLink.querySelector('img');
                if (avatarImg && avatarImg.src) {
                    isAuthenticated = true;
                    simpleSSO.avatar = avatarImg.src;
                }
                // користи innerText у случају да ThriveCart промени начин приказивања имена профила.
                if (profileLink.innerText) {
                    isAuthenticated = true;
                    simpleSSO.username = profileLink.innerText;
                } else {
                    const bold = profileLink.querySelector('b');
                    if (bold && bold.innerText) {
                        isAuthenticated = true;
                        simpleSSO.username = bold.innerText;
                    }
                }
            } else {
                if (!attemptsRemaining) {
                    return console.error('Could not load FastComments - could not determine user information (user name/avatar). Please reach out to FastComments support.');
                }
                console.warn('FastComments: No user profile info found - waiting and trying again.');
                attemptsRemaining--;
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // повећај време чекања након 5 покушаја у случају спорог интернета.
            }

            let url;
            const selectedNavLink = document.querySelector('.tcc-browse-lesson.active a');

            if (selectedNavLink) {
                // понекад TC користи више веза за исту страницу, па уклонимо дупликате.
                url = getPathnameFromUrl(selectedNavLink.href);
            } else {
                // уклони маркетиншке параметре и име домена
                url = window.location.pathname;
            }

            if (url) {
                url = url.replace('/starte-hier', '');
                url = url.replace('/start-here', '');
            }

            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: 'demo',
                urlId: url,
                simpleSSO: isAuthenticated ? simpleSSO : null
            });
        }

        tryLoad();

        function getPathnameFromUrl(url) {
            try {
                const parsedUrl = new URL(url);
                // уклони маркетиншке параметре и име домена
                return parsedUrl.pathname;
            } catch (error) {
                console.error("Invalid URL", url, error);
                return window.location.pathname; // подразумевано на текућу, тако да барем понекад функционише.
            }
        }

    })();
</script>
[inline-code-end]

Сада га налепите у блок кода са леве стране у ThriveCart едитору. Требало би да изгледа овако:

<div class="screenshot white-bg">
    <div class="title">Код додат</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-1-paste-code.png" alt="Код додат" />
</div>

То је то! Сада само треба да објавимо:

<div class="screenshot white-bg">
    <div class="title">Објави</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-2-publish.png" alt="Објави" />
</div>

То је то! Сада би требало да видите поље за коментаре на вашем курсу када га прегледате, а стварни корисници ће моћи да остављају коментаре **без пријављивања или уношења свог корисничког имена/е-поште по други пут**.

### Напомена за тестирање!

Ако имате онемогућено анонимно коментарисање, што је подразумевано, нећете моћи да остављате коментаре у режиму `Preview` као корисник `John Smith`. Добићете грешку аутентификације јер подразумевани корисник `John Smith` нема е-пошту. Ако желите да тестирате, предлажемо да користите купон (coupon) и прођете кроз ваш сајт као прави корисник.