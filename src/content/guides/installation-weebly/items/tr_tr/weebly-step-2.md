Weebly ile FastComments entegrasyonunun düzgün çalışması için iki küçük kod parçası eklememiz gerekiyor.

İlk kod parçası Weebly'deki "Comments are Closed" mesajını gizlemek içindir, ikincisi ise FastComments'i gerçekten yüklemektir.

İlk olarak, bu küçük kod parçasını kopyalayın:

[inline-code-attrs-start title = 'FastComments Üstbilgi Kod Parçacığı'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<style>
    #comments {
        display: none;
    }
    #commentArea:not(.loaded) {
        display: none;
    }
    #commentArea.loaded {
        display: block !important;
    }
</style>
[inline-code-end]

Daha sonra, `Step One`'dan gelen aynı ayarlar sayfasında, `Post header code`'un yanındaki `+`'a tıklayın.

<div class="screenshot white-bg">
    <div class="title">Gönderi Üstbilgi Kodunu Aç</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-1-open-post-header-code.png" alt="Gönderi Üstbilgi Kodunu Aç" />
</div>

Aşağıdaki gibi bir metin kutusunun açıldığını görmelisiniz:

<div class="screenshot white-bg">
    <div class="title">Gönderi Üstbilgi Kodu Açık</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-2-post-header-code-open.png" alt="Gönderi Üstbilgi Kodu Açık" />
</div>

Şimdi kod parçamızı yapıştıralım:

<div class="screenshot white-bg">
    <div class="title">Üstbilgi Kod Parçacığı Yapıştırıldı</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-3-post-header-code-added.png" alt="Üstbilgi Kod Parçacığı Yapıştırıldı" />
</div>

Sırada FastComments'i etkinleştirmek için altbilgi kodu var. `Post footer code`'un yanındaki artı işaretine tıklayın:

<div class="screenshot white-bg">
    <div class="title">Gönderi Altbilgi Kodunu Aç</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-4-open-post-footer-code.png" alt="Gönderi Altbilgi Kodunu Aç" />
</div>

Aşağıdaki kod parçacığını kopyalayın; bu kod **özellikle Weebly için** tasarlanmıştır:

[inline-code-attrs-start title = 'FastComments Altbilgi Kod Parçacığı'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        let loaded = false;
        let interval = null;
        function attemptLoad() {
            if (loaded) {
                clearInterval(interval);
                return;
            }
            const comments = document.getElementById('comments');
            if (comments) { // yorumları göster düğmesini kaldır
                comments.remove();
            }
            const commentArea = document.getElementById('commentArea');
            if (!commentArea) {
                return;
            }
            commentArea.innerHTML = '';
            commentArea.classList.add('loaded');
            FastCommentsUI(commentArea, {
                tenantId: "demo",
                urlId: window.location.pathname
            });
            loaded = true;
            clearInterval(interval);
        }
        attemptLoad();
        interval = setInterval(attemptLoad, 300);
    })();
</script>
[inline-code-end]

Şimdi altbilgi kodumuzu yapıştıralım:

<div class="screenshot white-bg">
    <div class="title">Gönderi Altbilgi Kodu Eklendi</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-5-post-footer-code-added.png" alt="Gönderi Altbilgi Kodu Eklendi" />
</div>

Bu kadar!