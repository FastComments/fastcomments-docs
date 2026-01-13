За Стъпка 2 трябва да копираме нашия фрагмент код. Проверете, че ред 50 не казва "demo" - ще искате това да съдържа вашия tenant id. Той трябва да бъде попълнен за вас.

Сега нека копираме нашия фрагмент с код на FastComments, специфичен за ThriveCart Learn.

Той е доста голям, защото интеграцията с ThriveCart има много функции, така че просто натиснете бутона Copy в горния десен ъгъл на фрагмента с код:

[inline-code-attrs-start title = 'Код за коментари за ThriveCart Learn+'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
                profileLink = document.querySelector('.thrivecart-courses-header-profile'); // класът е различен за визуализация.
            }
            // общ селектор за полето за имейл в случай, че ThriveCart промени id-то.
            const emailInputField = document.querySelector('input[type=email]');
            if (emailInputField && emailInputField.value) {
                isAuthenticated = true;
                simpleSSO.email = emailInputField.value;
            } else if (profileLink && !profileLink.innerText.includes('John Smith')) { // позволи на визуализацията да работи - няма наличен имейл.
                attemptsRemaining--;
                if (!attemptsRemaining) {
                    return console.error('Could not load FastComments - could not determine user information (email). Please reach out to FastComments support.');
                }
                console.warn('FastComments: No user email found - waiting and trying again.');
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // увеличи времето за изчакване след 5 опита в случай на бавен интернет.
            }
            if (profileLink) {
                // използвай директна заявка 'img' в случай, че ThriveCart промени селектора на изображението.
                const avatarImg = profileLink.querySelector('img');
                if (avatarImg && avatarImg.src) {
                    isAuthenticated = true;
                    simpleSSO.avatar = avatarImg.src;
                }
                // използвай innerText в случай, че ThriveCart промени начина, по който се показва името на профила.
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
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // увеличи времето за изчакване след 5 опита в случай на бавен интернет.
            }

            let url;
            const selectedNavLink = document.querySelector('.tcc-browse-lesson.active a');

            if (selectedNavLink) {
                // понякога TC използва няколко линка на една и съща страница, така че нека премахнем дубликатите.
                url = getPathnameFromUrl(selectedNavLink.href);
            } else {
                // премахни маркетинговите параметри и името на домейна
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
                // премахни маркетинговите параметри и името на домейна
                return parsedUrl.pathname;
            } catch (error) {
                console.error("Invalid URL", url, error);
                return window.location.pathname; // по подразбиране към текущата, така поне понякога ще работи
            }
        }

    })();
</script>
[inline-code-end]

Сега го поставете в блока за код вляво в редактора на ThriveCart. Трябва да изглежда така:

<div class="screenshot white-bg">
    <div class="title">Кодът е добавен</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-1-paste-code.png" alt="Кодът е добавен" />
</div>

Това е всичко! Сега просто трябва да публикуваме:

<div class="screenshot white-bg">
    <div class="title">Публикуване</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-2-publish.png" alt="Публикуване" />
</div>

Това е всичко! Трябва вече да виждате полето за коментари в курса при преглед, а реалните потребители ще могат да оставят коментари **без да влизат или да въвеждат отново потребителското си име/имейл**.

### Забележка за тестване!

Ако анонимното коментиране е изключено, което е по подразбиране, няма да можете да оставяте коментари в `Preview` режим като потребител `John Smith`. Ще получите грешка за удостоверяване
тъй като подразбиращият се потребител `John Smith` няма имейл. Ако искате да тествате, препоръчваме да използвате код за отстъпка и да преминете през сайта си като реален потребител.

---