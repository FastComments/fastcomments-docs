[related-parameter-start name = 'countAll'; type = 'boolean'; related-parameter-end]

Yorum widget'ının en üstünde görüntülenen yorum sayısı, ya tümüyle "üst düzey" yorumları — yani doğrudan sayfaya veya makaleye yapılan yanıtları — ya da **tüm** iç içe geçmiş yorumların sayısını gösterebilir.

Varsayılan olarak bu değer `true`'dur - yani ikincisinin sayacıdır - tüm yorumlar. Yorum widget'ının eski sürümlerinde varsayılan değer `false`'tur.

**countAll** bayrağını true olarak ayarlayarak davranışı değiştirebilir, böylece **tüm** iç içe geçmiş yorumların sayısını gösterebiliriz.

[code-example-start config = {countAll: true}; linesToHighlight = [6]; title = 'Tüm Yorumları Sayma'; code-example-end]

Sayının yalnızca üst düzey yorumları yansıtmasını istiyorsak, bayrağı false olarak ayarlarız.

[code-example-start config = {countAll: false}; linesToHighlight = [6]; title = 'Üst Düzey Yorumları Sayma'; code-example-end]

Bu şu anda kod değişikliği yapılmadan özelleştirilemez.