[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

Varsayılan olarak, FastComments yorumları "En İlgili" sıralama yönüne göre sıralar.

"En İlgili" sıralama, yorumun bırakıldığı zamanı ve oy sayısını sıralama için dikkate alır.

Kullanıcı, yorum widget'ı arayüzünde sıralama yönünü En Eski ya da En Yeni İlk olarak değiştirebilir.

Bununla birlikte, varsayılanı üçünden herhangi birine değiştirebiliriz. Örneğin, en eski yorumları önce göstermek isterseniz:

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Oldest First'; code-example-end]

**defaultSortDirection** değerini "OF" olarak ayarlayarak yönü "OF" olarak belirleriz.

En yeni önce sıralama yönü için aşağıdakini yaparız:

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Newest First'; code-example-end]

**defaultSortDirection** için geçerli değerler şunlardır:

- MR: "En Yeni"
- NF: "En Yeni İlk"
- OF: "En Eski İlk"

Bu, kod olmadan da yapılabilir. Widget özelleştirme sayfasında "Varsayılan Sıralama Yönü" bölümüne bakın.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; alt='Varsayılan Sıralama Yönü seçicisi, En İlgili, En Yeni İlk ve En Eski İlk seçeneklerini sunar'; title='Varsayılan Sıralama Yönünü Değiştirme' app-screenshot-end]

Not: Her sıralama yönü için her sayfadaki yorumlar önceden hesaplanır, bu yüzden tüm sıralama yönleri aynı performansa sahiptir.