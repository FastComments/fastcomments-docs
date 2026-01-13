Artık özel bir HTML bloğu eklediğinize göre, FastComments widget kodunu ekleyebiliriz.

**Aşağıdaki kodu Godaddy için kullanın, diğer öğreticilerdeki kodları kullanmayın. Bu kod Godaddy'ye özgüdür.**

Aşağıdaki kodu kopyalayın:

[inline-code-attrs-start title = 'Godaddy Yorum Kod Parçası'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        if (top.location.pathname && top.location.pathname.includes('/f')) {
            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: "demo",
                pageTitle: top.document.title,
                url: top.location.href,
                urlId: top.location.pathname
            });
        }
    })();
</script>
[inline-code-end]

Bu özel kod parçası Godaddy ile uyumlu olacak şekilde tasarlanmıştır ve yalnızca blog gönderilerinizde gösterilecek - ana sayfada değil.

Şimdi kodu `Step One`'da bahsedilen `Custom Code` alanına yapıştırın.

<div class="screenshot white-bg">
    <div class="title">Kodu Kopyalayıp Yapıştırın</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-code-added.png" alt="Kodu Kopyalayıp Yapıştırın" />
</div>

Sağ üst köşedeki Done'a tıklayın:

<div class="screenshot white-bg">
    <div class="title">Done'a Tıklayın</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-done.png" alt="Done'a Tıklayın" />
</div>

İkinci adım bu kadar!

---