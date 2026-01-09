Para o Passo 2, precisamos copiar nosso trecho de código. Verifique se a linha 50 não diz "demo" - você vai querer garantir que isso contenha o seu tenant id. Deve ser preenchido para você.

Agora vamos copiar nosso trecho de código FastComments específico para o ThriveCart Learn.

É bastante grande, porque a integração com o ThriveCart tem muitos recursos, então apenas clique no botão Copiar no canto superior direito do trecho de código:

[inline-code-attrs-start title = 'Código de comentários do ThriveCart Learn+'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
                profileLink = document.querySelector('.thrivecart-courses-header-profile'); // a classe é diferente para o preview.
            }
            // seletor amplo do campo de e-mail caso o ThriveCart mude o id.
            const emailInputField = document.querySelector('input[type=email]');
            if (emailInputField && emailInputField.value) {
                isAuthenticated = true;
                simpleSSO.email = emailInputField.value;
            } else if (profileLink && !profileLink.innerText.includes('John Smith')) { // permitir que o preview funcione - sem e-mail disponível.
                attemptsRemaining--;
                if (!attemptsRemaining) {
                    return console.error('Could not load FastComments - could not determine user information (email). Please reach out to FastComments support.');
                }
                console.warn('FastComments: No user email found - waiting and trying again.');
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // aumentar o tempo de espera após 5 tentativas caso a internet esteja lenta.
            }
            if (profileLink) {
                // usar a consulta "img" diretamente caso o ThriveCart altere o seletor de classe da imagem.
                const avatarImg = profileLink.querySelector('img');
                if (avatarImg && avatarImg.src) {
                    isAuthenticated = true;
                    simpleSSO.avatar = avatarImg.src;
                }
                // usar innerText caso o ThriveCart altere como o nome do perfil é exibido.
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
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // aumentar o tempo de espera após 5 tentativas caso a internet esteja lenta.
            }

            let url;
            const selectedNavLink = document.querySelector('.tcc-browse-lesson.active a');

            if (selectedNavLink) {
                // às vezes o TC usa múltiplos links na mesma página, então vamos remover duplicatas.
                url = getPathnameFromUrl(selectedNavLink.href);
            } else {
                // remover parâmetros de marketing e nome do domínio
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
                // remover parâmetros de marketing e nome do domínio
                return parsedUrl.pathname;
            } catch (error) {
                console.error("Invalid URL", url, error);
                return window.location.pathname; // default to current, so at least it works sometimes
            }
        }

    })();
</script>
[inline-code-end]

Agora cole-o no bloco de código à esquerda no editor do ThriveCart. Deve ficar assim:

<div class="screenshot white-bg">
    <div class="title">Código Adicionado</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-1-paste-code.png" alt="Código Adicionado" />
</div>

É isso! Agora só precisamos publicar:

<div class="screenshot white-bg">
    <div class="title">Publicar</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-2-publish.png" alt="Publicar" />
</div>

É isso! Agora você deve ver a caixa de comentários no seu curso ao visualizar em `Preview`, e usuários reais poderão deixar comentários **sem precisar fazer login ou fornecer seu nome de usuário/e-mail pela segunda vez**.

### Observação sobre testes!

Se você tiver comentários anônimos desabilitados, o que ocorre por padrão, você não poderá deixar comentários no modo `Preview` como o usuário `John Smith`. Você receberá um erro de autenticação
pois o usuário padrão `John Smith` não tem e-mail. Se quiser testar, sugerimos que use um código de cupom e navegue pelo seu site como um usuário real.