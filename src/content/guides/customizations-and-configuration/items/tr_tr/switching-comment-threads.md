[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

`urlId` değerinin, yorumların bağlı olduğu sayfa veya makale kimliği olduğunu ele almıştık.

Ayrıca, tekrar hatırlatmak gerekirse, tanımlanmadığı takdirde `urlId` varsayılan olarak geçerli sayfa URL'sini alır.

Peki, yorumların bağlı olduğu sayfa veya içerik sayfa yenilenmeden dinamik olarak değişen SPA'larda (Single-Page-Applications), yani Tek Sayfa Uygulamalarında durum nedir?

#### Angular, React, Vue, vb.

Angular ve React gibi kütüphanelerimizde, widget'a iletilen `urlId` özelliğini güncellemeniz yeterlidir; bu, yorum widget'ının yenilenmesine neden olur. Bunu, örneğin React uygulaması için <a href="https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx#L39" target="_blank">burada</a> iş başında görebilirsiniz.

#### VanillaJS

VanillaJS kütüphanesini kullanıyorsanız, Angular veya React gibi veri bağlamayı veya durum yayılımını yöneten bir framework olmadığı için bu biraz daha karmaşıktır.

VanillaJS widget'ını örneklediğinizde, widget'ı güncellemek için çağrılabilecek bazı fonksiyonlar döndürür.

İşte sayfa hash'ini değiştirdiğimiz ve yorum widget'ını güncellediğimiz işlevsel bir örnek:

[inline-code-attrs-start title = 'Sayfa Hash Değiştirme Örneği'; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<button id="change-page"></button>
<div id="fastcomments-widget"></div>
<script>
    (function fastCommentsMain() {
        let config = {
            tenantId: 'demo'
        };
        let instance = window.FastCommentsUI(document.getElementById('fastcomments-widget'), config);

        let page = '#page-1';
        function getNextPage() {
            if (page === '#page-1') {
                return '#page-2';
            } else {
                return '#page-1';
            }
        }

        let button = document.getElementById('change-page');
        function nextPage() {
            page = getNextPage();
            button.innerText = 'Go to ' + getNextPage();
            window.location.hash = page;
            let locationString = window.location.toString();

            config.url = locationString; // Bildirimlerin doğru sayfaya bağlanabilmesi için url değerini de güncelliyoruz
            config.urlId = locationString;

            instance.update(config);
        }
        nextPage();
        button.addEventListener('click', nextPage);
    })();
</script>
[inline-code-end]