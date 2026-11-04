Pour l'étape 2, nous devons copier notre extrait de code. Vérifiez que la ligne 50 ne dit pas "demo" - vous devez vous assurer qu'elle contient votre identifiant de tenant. Il devrait être pré-rempli pour vous.

Copions maintenant notre extrait de code FastComments spécifique à ThriveCart-Learn.

Il est assez volumineux, car l'intégration avec ThriveCart possède de nombreuses fonctionnalités, cliquez donc simplement sur le bouton Copier en haut à droite de l'extrait de code :

[inline-code-attrs-start title = 'Code de commentaires ThriveCart Learn+'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
                profileLink = document.querySelector('.thrivecart-courses-header-profile'); // la classe est différente pour l'aperçu.
            }
            // sélecteur large du champ de saisie e-mail au cas où ThriveCart changerait l'id.
            const emailInputField = document.querySelector('input[type=email]');
            if (emailInputField && emailInputField.value) {
                isAuthenticated = true;
                simpleSSO.email = emailInputField.value;
            } else if (profileLink && !profileLink.innerText.includes('John Smith')) { // permet à l'aperçu de fonctionner - aucun e-mail disponible.
                attemptsRemaining--;
                if (!attemptsRemaining) {
                    return console.error('Could not load FastComments - could not determine user information (email). Please reach out to FastComments support.');
                }
                console.warn('FastComments: No user email found - waiting and trying again.');
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // augmente le temps d'attente après 5 tentatives au cas où la connexion serait lente.
            }
            if (profileLink) {
                // utilise une requête "img" brute au cas où ThriveCart changerait le sélecteur de classe de l'image.
                const avatarImg = profileLink.querySelector('img');
                if (avatarImg && avatarImg.src) {
                    isAuthenticated = true;
                    simpleSSO.avatar = avatarImg.src;
                }
                // utilise innerText au cas où ThriveCart changerait la façon dont le nom du profil est affiché.
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
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // augmente le temps d'attente après 5 tentatives au cas où la connexion serait lente.
            }

            let url;
            const selectedNavLink = document.querySelector('.tcc-browse-lesson.active a');

            if (selectedNavLink) {
                // parfois TC utilise plusieurs liens pour la même page, donc dédupliquons-les.
                url = getPathnameFromUrl(selectedNavLink.href);
            } else {
                // supprime les paramètres marketing et le nom de domaine
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
                // supprime les paramètres marketing et le nom de domaine
                return parsedUrl.pathname;
            } catch (error) {
                console.error("Invalid URL", url, error);
                return window.location.pathname; // utilise la valeur actuelle par défaut, pour que cela fonctionne au moins parfois
            }
        }

    })();
</script>
[inline-code-end]

Collez-le maintenant dans le bloc de code à gauche dans l'éditeur ThriveCart. Cela devrait ressembler à ceci :

<div class="screenshot white-bg">
    <div class="title">Code ajouté</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-1-paste-code.png" alt="Code ajouté" />
</div>

Et voilà ! Il ne nous reste plus qu'à publier :

<div class="screenshot white-bg">
    <div class="title">Publier</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-2-publish.png" alt="Publier" />
</div>

Et voilà ! Vous devriez maintenant voir la boîte de commentaires sur votre cours lors de l'aperçu, et les vrais utilisateurs pourront laisser des commentaires **sans se connecter ni ressaisir leur nom d'utilisateur/e-mail une seconde fois**.

### Note de test !

Si vous avez désactivé les commentaires anonymes, ce qui est le cas par défaut, vous ne pourrez pas laisser de commentaires en mode `Preview` en tant qu'utilisateur `John Smith`. Vous obtiendrez une erreur
d'authentification car l'utilisateur par défaut `John Smith` n'a pas d'e-mail. Si vous souhaitez tester, nous vous suggérons d'utiliser un code promo et de parcourir votre site comme un véritable utilisateur.