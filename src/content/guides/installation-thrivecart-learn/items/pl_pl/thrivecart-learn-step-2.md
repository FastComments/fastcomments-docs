W Kroku 2 musimy skopiować nasz fragment kodu. Sprawdź, czy w linii 50 nie ma napisu "demo" - upewnij się, że znajduje się tam Twój identyfikator tenant. Powinien być wypełniony automatycznie.

Teraz skopiujmy nasz fragment kodu FastComments specyficzny dla ThriveCart-Learn.

Jest dość duży, ponieważ integracja z ThriveCart ma wiele funkcji, więc po prostu kliknij przycisk Copy w prawym górnym rogu fragmentu kodu:

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
            // szeroki selektor pola e-mail na wypadek zmiany id przez ThriveCart.
            const emailInputField = document.querySelector('input[type=email]');
            if (emailInputField && emailInputField.value) {
                isAuthenticated = true;
                simpleSSO.email = emailInputField.value;
            } else if (profileLink && !profileLink.innerText.includes('John Smith')) { // pozwól, aby podgląd działał - brak dostępnego e-maila.
                attemptsRemaining--;
                if (!attemptsRemaining) {
                    return console.error('Could not load FastComments - could not determine user information (email). Please reach out to FastComments support.');
                }
                console.warn('FastComments: No user email found - waiting and trying again.');
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // zwiększ czas oczekiwania po 5 próbach na wypadek wolnego internetu.
            }
            if (profileLink) {
                // użyj surowego zapytania "img" na wypadek zmiany selektora klasy obrazu przez ThriveCart.
                const avatarImg = profileLink.querySelector('img');
                if (avatarImg && avatarImg.src) {
                    isAuthenticated = true;
                    simpleSSO.avatar = avatarImg.src;
                }
                // użyj innerText na wypadek zmiany sposobu wyświetlania nazwy profilu przez ThriveCart.
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
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // zwiększ czas oczekiwania po 5 próbach na wypadek wolnego internetu.
            }

            let url;
            const selectedNavLink = document.querySelector('.tcc-browse-lesson.active a');

            if (selectedNavLink) {
                // czasami TC używa wielu linków do tej samej strony, więc usuńmy duplikaty.
                url = getPathnameFromUrl(selectedNavLink.href);
            } else {
                // przytnij parametry marketingowe i nazwę domeny
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
                // przytnij parametry marketingowe i nazwę domeny
                return parsedUrl.pathname;
            } catch (error) {
                console.error("Invalid URL", url, error);
                return window.location.pathname; // domyślnie bieżąca, aby przynajmniej czasami działało
            }
        }

    })();
</script>
[inline-code-end]

Teraz wklej go do bloku kodu po lewej stronie w edytorze ThriveCart. Powinno to wyglądać tak:

<div class="screenshot white-bg">
    <div class="title">Kod dodany</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-1-paste-code.png" alt="Kod dodany" />
</div>

To wszystko! Teraz musimy tylko opublikować:

<div class="screenshot white-bg">
    <div class="title">Publikuj</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-2-publish.png" alt="Publikuj" />
</div>

Gotowe! Powinieneś teraz zobaczyć pole komentarzy w swoim kursie podczas podglądu, a prawdziwi użytkownicy będą mogli zostawiać komentarze **bez logowania się ani ponownego podawania nazwy użytkownika/e-maila**.

### Uwaga dotycząca testowania!

Jeśli masz wyłączone anonimowe komentowanie (co jest domyślne), nie będziesz mógł zostawiać komentarzy w trybie `Preview` jako użytkownik `John Smith`. Otrzymasz błąd uwierzytelniania, ponieważ domyślny użytkownik `John Smith` nie ma adresu e-mail. Jeśli chcesz przetestować, sugerujemy użycie kodu kuponu i przejście przez swoją stronę jak prawdziwy użytkownik.