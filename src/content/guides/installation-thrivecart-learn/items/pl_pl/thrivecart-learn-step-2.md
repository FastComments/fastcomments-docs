Na Kroku 2 musimy skopiować nasz fragment kodu. Sprawdź, czy w linii 50 nie ma napisu "demo" - upewnij się, że znajduje się tam Twój tenant id. Powinno być wypełnione dla Ciebie.

Teraz skopiujmy nasz fragment kodu FastComments specyficzny dla ThriveCart Learn.

Jest dość duży, ponieważ integracja z ThriveCart ma wiele funkcji, więc po prostu kliknij przycisk Kopiuj w prawym górnym rogu fragmentu kodu:

[inline-code-attrs-start title = 'Kod komentarzy ThriveCart Learn+'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
                profileLink = document.querySelector('.thrivecart-courses-header-profile'); // klasa jest inna w podglądzie.
            }
            // szeroki selektor pola e-mail na wypadek, gdy ThriveCart zmieni id.
            const emailInputField = document.querySelector('input[type=email]');
            if (emailInputField && emailInputField.value) {
                isAuthenticated = true;
                simpleSSO.email = emailInputField.value;
            } else if (profileLink && !profileLink.innerText.includes('John Smith')) { // pozwól, by podgląd działał — brak dostępnego e-maila.
                attemptsRemaining--;
                if (!attemptsRemaining) {
                    return console.error('Could not load FastComments - could not determine user information (email). Please reach out to FastComments support.');
                }
                console.warn('FastComments: No user email found - waiting and trying again.');
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // zwiększ czas oczekiwania po 5 próbach, na wypadek wolnego internetu.
            }
            if (profileLink) {
                // użyj surowego zapytania "img" na wypadek, gdy ThriveCart zmieni selektor klasy obrazu.
                const avatarImg = profileLink.querySelector('img');
                if (avatarImg && avatarImg.src) {
                    isAuthenticated = true;
                    simpleSSO.avatar = avatarImg.src;
                }
                // użyj innerText na wypadek, gdy ThriveCart zmieni sposób wyświetlania nazwy profilu.
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
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // zwiększ czas oczekiwania po 5 próbach, na wypadek wolnego internetu.
            }

            let url;
            const selectedNavLink = document.querySelector('.tcc-browse-lesson.active a');

            if (selectedNavLink) {
                // czasami TC używa wielu linków do tej samej strony, więc usuńmy duplikaty.
                url = getPathnameFromUrl(selectedNavLink.href);
            } else {
                // usuń parametry marketingowe i nazwę domeny
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
                // usuń parametry marketingowe i nazwę domeny
                return parsedUrl.pathname;
            } catch (error) {
                console.error("Invalid URL", url, error);
                return window.location.pathname; // domyślnie do bieżącej, żeby przynajmniej czasem działało
            }
        }

    })();
</script>
[inline-code-end]

Now paste it into the code block on the left in the ThriveCart editor. It should look like this:

<div class="screenshot white-bg">
    <div class="title">Kod dodany</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-1-paste-code.png" alt="Kod dodany" />
</div>

That's it! Now we just have to publish:

<div class="screenshot white-bg">
    <div class="title">Opublikuj</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-2-publish.png" alt="Opublikuj" />
</div>

That's it! You should now see the comment box on your course when you preview, and real users will be able to leave comments **without signing in or leaving their username/email a second time**.

### Testing Note!

If you have anonymous commenting disabled, which it is by default, you won't be able to leave comments in `Preview` mode as the `John Smith` user. You will get an authentication
error as the default `John Smith` user has no email. If you want to test, we suggest you use a coupon code and go through your site like an actual user.

---