На кроці 2 ми маємо скопіювати наш фрагмент коду. Перевірте, що в рядку 50 не вказано "demo" - вам потрібно переконатися, що там ваш tenant id. Він має бути підставлений для вас.

Тепер скопіюємо наш специфічний для ThriveCart Learn фрагмент коду FastComments.

Він досить великий, оскільки інтеграція з ThriveCart має багато функцій, тому просто натисніть кнопку «Копіювати» у верхньому правому куті фрагмента коду:

[inline-code-attrs-start title = 'Код коментарів ThriveCart Learn+'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
                profileLink = document.querySelector('.thrivecart-courses-header-profile'); // клас відрізняється для режиму попереднього перегляду.
            }
            // загальний селектор поля вводу email на випадок, якщо ThriveCart змінить id.
            const emailInputField = document.querySelector('input[type=email]');
            if (emailInputField && emailInputField.value) {
                isAuthenticated = true;
                simpleSSO.email = emailInputField.value;
            } else if (profileLink && !profileLink.innerText.includes('John Smith')) { // дозволити попередньому перегляду працювати — електронна пошта недоступна.
                attemptsRemaining--;
                if (!attemptsRemaining) {
                    return console.error('Could not load FastComments - could not determine user information (email). Please reach out to FastComments support.');
                }
                console.warn('FastComments: No user email found - waiting and trying again.');
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // збільшити час очікування після 5 спроб на випадок повільного інтернету.
            }
            if (profileLink) {
                // використати безпосередній селектор "img" на випадок, якщо ThriveCart змінить селектор класу зображення.
                const avatarImg = profileLink.querySelector('img');
                if (avatarImg && avatarImg.src) {
                    isAuthenticated = true;
                    simpleSSO.avatar = avatarImg.src;
                }
                // використовувати innerText на випадок, якщо ThriveCart змінить спосіб відображення імені профілю.
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
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // збільшити час очікування після 5 спроб на випадок повільного інтернету.
            }

            let url;
            const selectedNavLink = document.querySelector('.tcc-browse-lesson.active a');

            if (selectedNavLink) {
                // іноді TC використовує кілька посилань на одній сторінці, тому давайте усунемо дублікати.
                url = getPathnameFromUrl(selectedNavLink.href);
            } else {
                // обрізати маркетингові параметри та назву домену
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
                // обрізати маркетингові параметри та назву домену
                return parsedUrl.pathname;
            } catch (error) {
                console.error("Invalid URL", url, error);
                return window.location.pathname; // за замовчуванням повернути поточний шлях, щоб принаймні іноді працювало
            }
        }

    })();
</script>
[inline-code-end]

Тепер вставте його в блок коду зліва в редакторі ThriveCart. Це має виглядати так:

<div class="screenshot white-bg">
    <div class="title">Код додано</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-1-paste-code.png" alt="Код додано" />
</div>

Ось і все! Тепер потрібно лише опублікувати:

<div class="screenshot white-bg">
    <div class="title">Опублікувати</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-2-publish.png" alt="Опублікувати" />
</div>

Ось і все! Тепер ви повинні бачити поле коментарів на вашому курсі під час попереднього перегляду, і реальні користувачі зможуть залишати коментарі **без повторного входу або повторного введення свого імені користувача/електронної пошти**.

### Примітка щодо тестування!

Якщо у вас вимкнено анонімні коментарі, а за замовчуванням вони вимкнені, ви не зможете залишити коментарі в режимі `Preview` як користувач `John Smith`. Ви отримаєте помилку автентифікації, оскільки в стандартного користувача `John Smith` відсутня електронна пошта. Якщо хочете протестувати, радимо використати купон і пройти через сайт як реальний користувач.