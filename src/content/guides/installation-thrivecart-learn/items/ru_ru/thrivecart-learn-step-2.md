Для шага 2 нам нужно скопировать наш фрагмент кода. Проверьте, что строка 50 не содержит "demo" — убедитесь, что там указан ваш tenant id. Он должен быть заполнен автоматически.

Теперь скопируем наш специфичный для ThriveCart-Learn фрагмент кода FastComments.

Он достаточно большой, потому что интеграция с ThriveCart имеет множество функций, поэтому просто нажмите кнопку Copy в правом верхнем углу фрагмента кода:

[inline-code-attrs-start title = 'Код комментариев ThriveCart Learn+'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
                profileLink = document.querySelector('.thrivecart-courses-header-profile'); // класс отличается для превью.
            }
            // общий селектор поля ввода email на случай, если ThriveCart изменит id.
            const emailInputField = document.querySelector('input[type=email]');
            if (emailInputField && emailInputField.value) {
                isAuthenticated = true;
                simpleSSO.email = emailInputField.value;
            } else if (profileLink && !profileLink.innerText.includes('John Smith')) { // разрешить работу превью — email отсутствует.
                attemptsRemaining--;
                if (!attemptsRemaining) {
                    return console.error('Could not load FastComments - could not determine user information (email). Please reach out to FastComments support.');
                }
                console.warn('FastComments: No user email found - waiting and trying again.');
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // увеличить время ожидания после 5 попыток на случай медленного интернета.
            }
            if (profileLink) {
                // использовать прямой поиск "img" на случай, если ThriveCart изменит селектор класса изображения.
                const avatarImg = profileLink.querySelector('img');
                if (avatarImg && avatarImg.src) {
                    isAuthenticated = true;
                    simpleSSO.avatar = avatarImg.src;
                }
                // использовать innerText на случай, если ThriveCart изменит способ отображения имени профиля.
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
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // увеличить время ожидания после 5 попыток на случай медленного интернета.
            }

            let url;
            const selectedNavLink = document.querySelector('.tcc-browse-lesson.active a');

            if (selectedNavLink) {
                // иногда TC использует несколько ссылок на одной странице, поэтому удалим дубликаты.
                url = getPathnameFromUrl(selectedNavLink.href);
            } else {
                // обрезать маркетинговые параметры и имя домена
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
                // обрезать маркетинговые параметры и имя домена
                return parsedUrl.pathname;
            } catch (error) {
                console.error("Invalid URL", url, error);
                return window.location.pathname; // по умолчанию текущая, чтобы хотя бы иногда работало
            }
        }

    })();
</script>
[inline-code-end]

Теперь вставьте его в блок кода слева в редакторе ThriveCart. Это должно выглядеть так:

<div class="screenshot white-bg">
    <div class="title">Код добавлен</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-1-paste-code.png" alt="Код добавлен" />
</div>

Вот и всё! Теперь нам осталось только опубликовать:

<div class="screenshot white-bg">
    <div class="title">Опубликовать</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-2-publish.png" alt="Опубликовать" />
</div>

Вот и всё! Теперь вы должны увидеть поле для комментариев на вашем курсе при предварительном просмотре, а реальные пользователи смогут оставлять комментарии **без повторного входа или повторного ввода имени пользователя/электронной почты**.

### Примечание по тестированию!

Если у вас отключены анонимные комментарии, что так по умолчанию, вы не сможете оставлять комментарии в режиме `Preview` как пользователь `John Smith`. Вы получите ошибку аутентификации
поскольку у пользователя по умолчанию `John Smith` нет email. Если вы хотите протестировать, мы рекомендуем воспользоваться купонным кодом и пройти по сайту как реальный пользователь.