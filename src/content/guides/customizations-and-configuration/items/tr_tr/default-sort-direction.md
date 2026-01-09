[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

Varsayılan olarak, FastComments yorumları "Most Relevant" sıralama yönüne göre sıralar.

Most Relevant sıralaması, yorumun bırakıldığı zamanı ve oy sayısını sıralama için dikkate alır.

Kullanıcı daha sonra yorum widget'ı kullanıcı arayüzünde sıralama yönünü Ya En Eski İlk ya da En Yeni İlk olarak değiştirebilir.

Ancak varsayılanı bu üçünden herhangi biri olacak şekilde değiştirebiliriz. Örneğin, en eski yorumları ilk göstermek isterseniz:

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Oldest First'; code-example-end]

**defaultSortDirection** değerini yönü "OF" olacak şekilde "OF" olarak ayarlarız.

En Yeni İlk sıralama yönü için aşağıdakileri yaparız:

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Newest First'; code-example-end]

**defaultSortDirection** için geçerli değerler şunlardır:

- MR: "Most Recent"
- NF: "Newest First"
- OF: "Oldest First"

Bu, kod olmadan da yapılabilir. Widget özelleştirme sayfasında, "Varsayılan Sıralama Yönü" bölümüne bakın.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; title='Changing The Default Sort Direction' app-screenshot-end]

Dikkat: Her sayfa için, her sıralama yönündeki yorumlar önceden hesaplanır; bu nedenle tüm sıralama yönleri aynı performansa sahiptir.