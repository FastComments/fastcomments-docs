Adım 2 için kod parçacığımızı kopyalamamız gerekiyor. 50. satırın "demo" demediğini kontrol edin - bunun kiracı kimliğinizi içermesini isteyeceksiniz. Bu sizin için doldurulmuş olmalı.

Şimdi ThriveCart-Learn'e özgü FastComments kod parçacığımızı kopyalayalım.

Bu oldukça büyük, çünkü ThriveCart ile entegrasyonun birçok özelliği var, bu yüzden kod parçacığının sağ üst köşesindeki Kopyala düğmesine tıklayın:

[inline-code-attrs-start title = 'ThriveCart Learn+ Yorumlar Kodu'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
                profileLink = document.querySelector('.thrivecart-courses-header-profile'); // önizleme için sınıf farklı.
            }
            // ThriveCart id'sini değiştirirse diye genel e-posta giriş alanı seçici.
            const emailInputField = document.querySelector('input[type=email]');
            if (emailInputField && emailInputField.value) {
                isAuthenticated = true;
                simpleSSO.email = emailInputField.value;
            } else if (profileLink && !profileLink.innerText.includes('John Smith')) { // önizlemenin çalışmasına izin ver - e-posta yok.
                attemptsRemaining--;
                if (!attemptsRemaining) {
                    return console.error('Could not load FastComments - could not determine user information (email). Please reach out to FastComments support.');
                }
                console.warn('FastComments: No user email found - waiting and trying again.');
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // internet yavaşsa, 5 denemeden sonra bekleme süresini artır.
            }
            if (profileLink) {
                // ThriveCart resim sınıfı seçicisini değiştirirse diye ham "img" sorgusunu kullan.
                const avatarImg = profileLink.querySelector('img');
                if (avatarImg && avatarImg.src) {
                    isAuthenticated = true;
                    simpleSSO.avatar = avatarImg.src;
                }
                // ThriveCart profil adını gösterme şeklini değiştirirse diye innerText kullan.
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
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // internet yavaşsa, 5 denemeden sonra bekleme süresini artır.
            }

            let url;
            const selectedNavLink = document.querySelector('.tcc-browse-lesson.active a');

            if (selectedNavLink) {
                // bazen TC aynı sayfada birden fazla bağlantı kullanır, bu yüzden bunları benzersizleştirelim.
                url = getPathnameFromUrl(selectedNavLink.href);
            } else {
                // pazarlama parametrelerini ve alan adını temizle
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
                // pazarlama parametrelerini ve alan adını temizle
                return parsedUrl.pathname;
            } catch (error) {
                console.error("Invalid URL", url, error);
                return window.location.pathname; // varsayılan olarak mevcut sayfayı kullan, böylece en azından bazen çalışır
            }
        }

    })();
</script>
[inline-code-end]

Şimdi bunu ThriveCart editöründeki sol taraftaki kod bloğuna yapıştırın. Şunun gibi görünmelidir:

<div class="screenshot white-bg">
    <div class="title">Kod Eklendi</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-1-paste-code.png" alt="Kod Eklendi" />
</div>

Hepsi bu! Şimdi sadece yayımlamamız gerekiyor:

<div class="screenshot white-bg">
    <div class="title">Yayımla</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-2-publish.png" alt="Yayımla" />
</div>

Hepsi bu kadar! Önizlemede kursunuzda yorum kutusunu görmelisiniz ve gerçek kullanıcılar **oturum açmadan veya kullanıcı adlarını/e-posta adreslerini ikinci kez girmeden** yorum bırakabilecekler.

### Test Notu!

Eğer anonim yorum yapma kapalıysa (ki varsayılan olarak kapalıdır), `Preview` modunda `John Smith` kullanıcısı olarak yorum bırakamazsınız. Varsayılan `John Smith` kullanıcısının e-posta adresi olmadığı için bir kimlik doğrulama hatası alırsınız. Test etmek istiyorsanız, bir kupon kodu kullanmanızı ve sitenizde gerçek bir kullanıcı gibi gezmenizi öneriyoruz.