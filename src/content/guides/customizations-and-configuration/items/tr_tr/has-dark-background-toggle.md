[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

İlk sayfa yüklemesinden sonra koyu modun açılıp kapatılmasına izin veren sitelerde, bu biraz daha fazla işlem gerektirir.

Öncelikle, Yorum widget kütüphanesinin (React, Vue) mevcut tüm sürümleri, koyu modun açılıp kapatılmasına örnekler içerir.

VanillaJS widget için biraz daha fazla çalışmamız gerekecek. Öncelikle, FastCommentsUI "destroy" ve "update" işlevlerine sahip bir nesne döndürür.

Yorum widget yapılandırmasını güncellemek istediğimiz her seferinde update işlevini basitçe çağırabiliriz, şöyle. İşte VanillaJS widget ile koyu modu açıp kapatmanın
tam çalışır bir örneği.

[inline-code-attrs-start title = 'Koyu Modu Açıp Kapatma Tam Örneği'; inline-code-attrs-end]
[inline-code-start]
<script src="https://fastcomments.com/js/embed-v2.min.js"></script>
<button id="toggle-dark-mode">Toggle Dark Mode</button>
<div id="fastcomments-widget"></div>
<script>
    (function() {
        const button = document.getElementById('toggle-dark-mode');
        const config = {
            tenantId: 'demo',
            hasDarkBackground: false
        };
        const instance = window.FastCommentsUI(document.getElementById('fastcomments-widget'), config);
        button.addEventListener('click', function() {
            config.hasDarkBackground = !config.hasDarkBackground;
            if (config.hasDarkBackground) {
                document.body.classList.add('dark');
            } else {
                document.body.classList.remove('dark');
            }
            instance.update(config);
        });
    })();
</script>
<style>
    body.dark {
        background: #000;
        color: #fff;
    }
</style>
[inline-code-end]

---