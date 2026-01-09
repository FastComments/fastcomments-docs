[related-parameter-start name = 'countAll'; type = 'boolean'; related-parameter-end]

Yorum aracının en üstünde gösterilen yorum sayısı ya tüm "top-level" yorumları gösterebilir; yani doğrudan sayfaya veya makaleye yapılan yanıtları, ya da **tüm** iç içe geçmiş yorumların sayısı olabilir.

Varsayılan olarak bu `true`'dur - bu, ikinci türün yani tüm yorumların sayısıdır. Eski sürümlerde yorum aracının varsayılan değeri `false` idi.

We can change the behavior, so that it is a count of **all** nested comments by setting the **countAll** flag to true.

[code-example-start config = {countAll: true}; linesToHighlight = [6]; title = 'Counting All Comments'; code-example-end]

Eğer sayının yalnızca üst seviye yorumları yansıtmasını isteseydik, bayrağı `false` olarak ayarlarız.

[code-example-start config = {countAll: false}; linesToHighlight = [6]; title = 'Counting Top Level Comments'; code-example-end]

Bu şu anda kod değişiklikleri olmadan özelleştirilemiyor.