[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Bir yorum dizisini görüntülerken veya yorum bırakırken, FastComments'in bu yorumların hangi sayfaya, makaleye veya ürüne ait olduğunu bilmesi gerekir.

Bunu yapmak için, "URL ID" adını verdiğimiz bir şey kullanırız. Bu, bir dize veya sayı gibi bir tanımlayıcı ya da bir URL olabilir.

Varsayılan olarak, urlId belirtmezseniz, sayfa URL'si haline gelir. Mevcut sayfa URL'sini alır ve yaygın pazarlama parametrelerini veya takip tanımlayıcılarını temizleriz.

WordPress gibi üçüncü taraf entegrasyonları söz konusu olduğunda, eklentimiz genellikle görüntülenmekte olan mevcut bilgiyi temsil eden tanımlayıcıyı URL ID olarak kullanır, örneğin makale/sayfa kimliği.

[code-example-start config = {urlId: 'https://example.com/page'}; linesToHighlight = [6]; title = 'Özel Bir URL ID Tanımlama'; code-example-end]

Bu dokümanda sıkça referans vereceğimiz şeylerden biri <a href="https://fastcomments.com/auth/my-account/customize-widget/new">Widget Özelleştirme Arayüzü</a>'dür.

Bu arayüz, kod kullanmadan yorum widget'ında birçok değişiklik yapmak için kullanılabilir.

Bir özelleştirme kuralı oluştururken, genellikle bunun sitemizdeki tüm sayfalara uygulanmasını isteriz. Ancak bazı durumlarda, yorum widget'ını belirli bir sayfada özelleştirmek isteriz; ya özel stil uygulamak ya da belki o belirli sayfa için yorumları anonim yapmak için. Örneğin, bazı sayfalarda canlı yorumların hemen görünmesini, diğerlerinde ise bildirim düğmelerinin altında gizlenmesini de sağlayabilirsiniz.

Tüm bunlar, bu sayfadaki URL ID giriş alanı aracılığıyla mümkündür ve şu şekilde görünür:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.url-id'; alt='Bir özelleştirme kuralını tek bir sayfaya veya */blog/* gibi bir desene kapsamak için kullanılan URL ID alanı'; title='Widget Özelleştirme Sayfasında URL ID Girişi' app-screenshot-end]

Bu alandaki değer, yorum widget'ına geçirilen *urlId* parametresiyle eşleşmelidir. Özelleştirme kuralınızın *urlId*'den bağımsız olmasını istiyorsanız, bu alanı boş bırakın veya * girin.

2023 itibarıyla, widget özelleştirmedeki `URL ID` alanı artık desenleri de kabul ediyor! Örneğin, blogunuza özgü stil eklemek için `*/blog/*` ve mağazanıza özgü stil için `*/store/*` kullanabilirsiniz, tüm bunları aynı alan adıyla yaparken.

### Dikkat Edilmesi Gerekenler

1. Sayfanızda hash parametreleri varsa (örneğin example.com#page-1) - bu, varsayılan olarak URL ID'nin bir parçası haline gelir.
2. Geçişler sırasında, örneğin WordPress'ten Gatsby'ye, ilk geçişten sonra URL ID yorum değerlerini taşımanız gerekebilir. Bunun için bize ulaşın.