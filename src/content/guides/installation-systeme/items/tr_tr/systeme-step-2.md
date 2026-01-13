---
Şimdi kodumuzu kopyalayacağız. Eğer kod parçacığında 6. satırda `tenantId: "demo"` yazıyorsa FastComments hesabınıza giriş yapmalı
ve sonra kopyalanan kod parçacığının hesap kimliğinizi içermesi için bu sayfayı yenilemelisiniz.

[inline-code-attrs-start title = 'Systeme.io Parçacığı'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo"
    });
</script>
[inline-code-end]

Şimdi bunu editöre yapıştırın ve kaydet'e tıklayın:

<div class="screenshot white-bg">
    <div class="title">FastComments Kodunu Ekle</div>
    <img class="screenshot-image" src="/images/installation-guides/systeme-add-code.png" alt="FastComments Kodunu Ekle" />
</div>

... sonra sitenizi kaydedin. Bu kadar!

---