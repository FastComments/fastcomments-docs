---
Şimdi kodumuzu kopyalayacağız. Eğer kod kesitinde 6. satırda `tenantId: "demo"` yazıyorsa FastComments hesabınıza giriş yapın
ve ardından kopyalanan kod kesitinin hesap kimliğinizi içermesi için bu sayfayı yenileyin.

[inline-code-attrs-start title = 'Systeme.io Kesiti'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo"
    }];
</script>
[inline-code-end]

Şimdi bunu editöre yapıştırın ve kaydet'e tıklayın:

<div class="screenshot white-bg">
    <div class="title">FastComments Kodunu Ekle</div>
    <img class="screenshot-image" src="/images/installation-guides/systeme-add-code.png" alt="FastComments Kodunu Ekle" />
</div>

... sonra sitenizi kaydedin. Hepsi bu!

---