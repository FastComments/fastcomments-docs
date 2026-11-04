[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Bu API, bir kullanıcıya gösterilmek üzere yorumları almak için kullanılır. Örneğin, onaylanmamış veya spam yorumları otomatik olarak filtreler.

### Sayfalama

Sayfalama, performans gereksinimlerine ve kullanım senaryosuna bağlı olarak iki yöntemden biriyle yapılabilir:

1. En Hızlı: **Önceden Hesaplanmış Sayfalama (Precalculated Pagination)**:
   1. FastComments, hazır widget ve istemcilerimizi kullandığınızda bu şekilde çalışır.
   2. "Sonraki"ye tıklamak yalnızca sayfa sayısını artırır.
   3. Bunu bir anahtar-değer deposundan alınmış gibi düşünebilirsiniz.
   4. Bu şekilde, `0` ile başlayan bir `page` parametresi ve `direction` olarak bir sıralama yönü tanımlamanız yeterlidir.
   5. Sayfa boyutları özelleştirme kuralları aracılığıyla özelleştirilebilir.
2. En Esnek: **Esnek Sayfalama (Flexible Pagination)**:
   1. Bu şekilde özel `limit` ve `skip` parametreleri tanımlayabilirsiniz. `page` parametresini geçmeyin.
   2. Sıralama `direction` da desteklenir.
   3. `limit`, `skip` uygulandıktan sonra döndürülecek toplam sayıdır.
      - Örnek: `page size = 100` ve `page = 2` iken `skip = 200, limit = 100` olarak ayarlayın.
   4. Alt yorumlar sayfalamaya dahil edilir. Bunu `asTree` seçeneğini kullanarak aşabilirsiniz.
      - Alt yorumları `limitChildren` ve `skipChildren` ile sayfalayabilirsiniz.
      - Döndürülen thread'lerin derinliğini `maxTreeDepth` ile sınırlayabilirsiniz.

### Thread'ler

1. `Precalculated Pagination` kullanırken, yorumlar *sayfa* bazında gruplanır ve thread'lerdeki yorumlar genel sayfayı etkiler.
   1. Bu şekilde, thread'ler istemci tarafında `parentId` temel alınarak belirlenebilir.
   2. Örneğin, bir üst düzey yorum ve 29 yanıtı olan bir sayfada API'ye `page=0` geçirerek - yalnızca üst düzey yorumu ve 29 alt yorumu alırsınız.
2. `Flexible Pagination` kullanırken bir `parentId` parametresi tanımlayabilirsiniz.
   1. Yalnızca üst düzey yorumları almak için bunu null olarak ayarlayın.
   2. Ardından thread'leri görüntülemek için API'yi tekrar çağırın ve `parentId` geçirin.
   3. Yaygın bir çözüm, üst düzey yorumlar için bir API çağrısı yapmak ve ardından her yorumun alt yorumlarını almak için paralel API çağrıları yapmaktır.
3. __Şubat 2023 itibarıyla YENİ!__ `&asTree=true` kullanarak ağaç olarak alın.
   1. Bunu `Flexible Pagination as a Tree` olarak düşünebilirsiniz.
   2. Sayfalamada yalnızca üst düzey yorumlar sayılır.
   3. Ağacı kökten başlatmak için `parentId=null` ayarlayın (`parentId` ayarlamanız gerekir).
   4. Sayfalama için `skip` ve `limit` ayarlayın.
   5. `asTree` değerini `true` yapın.
   6. Kredi maliyeti `2x` artar, çünkü backend'imizin bu senaryoda çok daha fazla iş yapması gerekir.
   7. `maxTreeDepth`, `limitChildren` ve `skipChildren` değerlerini istediğiniz gibi ayarlayın.

### Ağaçlar Açıklandı

`asTree` kullanırken sayfalama hakkında akıl yürütmek zor olabilir. İşte faydalı bir grafik:

<div class="screenshot white-bg">
    <div class="title">Ağaç Sayfalama Diyagramı</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Ağaç Sayfalama Diyagramı" />
</div>

### Kullanıcı Bağlamında Yorumları Alma

`/comments` API'si, farklı kullanım senaryoları için iki bağlamda kullanılabilir:

- Kendi istemcinizi oluşturmak için sıralanmış ve bilgiyle etiketlenmiş yorumlar döndürmek için.
  - Bu durumda bir `contextUserId` sorgu parametresi tanımlayın.
- Özel entegrasyonlar için yorumları backend'inizden almak için.
  - Platform, `contextUserId` olmadan varsayılan olarak bunu kullanır.

[inline-code-attrs-start title = 'Yorumlar Önceden Hesaplanmış Sayfalama'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'Yorumlar Esnek Sayfalama'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'Kullanıcı Bağlamında Yorumlar Esnek Sayfalama'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Yalnızca Üst Düzey Yorumlar İçin Kullanıcı Bağlamında Yorumlar Esnek Sayfalama'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Yorumları Ağaç Olarak Alma

Yorumların ağaç olarak döndürülmesi, sayfalamanın yalnızca üst düzey yorumları saymasıyla mümkündür.

[inline-code-attrs-start title = 'Kullanıcı Bağlamında Ağaç Olarak Yorumlar'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Yalnızca üst düzey yorumları ve birinci derece alt yorumları mı almak istiyorsunuz? İşte bir yol:

[inline-code-attrs-start title = 'Maksimum Derinlikli Ağaç Olarak Yorumlar'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

Ancak, arayüzünüzde her yorumda "yanıtları göster" düğmesinin gösterilip gösterilmeyeceğini bilmeniz gerekebilir. Yorumları ağaç olarak alırken, uygulanabilir olduğunda yorumlara bir `hasChildren` özelliği etiketlenir.

### Yorumları Ağaç Olarak, Hash Tag ile Arama Yaparak Alma

API kullanarak tüm tenant'ınız genelinde (tek bir sayfayla veya `urlId` ile sınırlı olmaksızın) hashtag ile arama yapmak mümkündür.

Bu örnekte, `urlId` parametresini çıkarıyoruz ve birden fazla hashtag ile arama yapıyoruz. API yalnızca istenen tüm hashtag'lere sahip yorumları döndürür.

[inline-code-attrs-start title = 'Kullanıcı Bağlamında Hash Tag ile Ağaç Olarak Yorumlar'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### Tüm İstek Parametreleri

[inline-code-attrs-start title = 'Yorumlar İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Yorumların ilişkili olduğu urlId (sayfa URL'si veya makale kimliği). **/
    urlId?: string
    /** Döndürülen yorumları bu kullanıcıyla sınırla. **/
    userId?: string
    /** Hashtag ile arama yapmak için bunu kullanın. Birden fazla hashtag'in kesişimine inmek için &hashTag=a&hashTag=b yapın. **/
    hashTag?: string
    /** Sıralama yönü. Varsayılan MR (En Alakalı). Diğer seçenekler OF (Önce En Eski) ve NF (Önce En Yeni). **/
    direction?: 'MR' | 'OF' | 'NF'
    /** Önceden Hesaplanmış Sayfalama: Alınacak sayfa, 0 ile başlar. Tüm yorumlar için (250'ye kadar) -1 geçirin. **/
    page?: number
    /** Esnek Sayfalama: Kaç yorum döndürmeliyiz? **/
    limit?: number
    /** Esnek Sayfalama: Her üst yorum için kaç alt yorum döndürmeliyiz? **/
    limitChildren?: number
    /** Esnek Sayfalama: Kaç yorum atlamalıyız? **/
    skip?: number
    /** Esnek Sayfalama: Her üst yorum için kaç alt yorum atlamalıyız? **/
    skipChildren?: number
    /** Engellenen ve işaretlenen yorumları belirlemek için. **/
    contextUserId?: string
    /** Engellenen ve işaretlenen yorumları belirlemek için. **/
    anonUserId?: string
    /** Alt yorumları almak için. **/
    parentId?: string
    /** Ağaç olarak almak için. **/
    asTree?: boolean
    /** Ağaçta ne kadar derine veri döndürmeliyiz? 0 alt yorum döndürmez. 1 birinci derece alt yorumları döndürür, vb. **/
    maxTreeDepth?: number
}
[inline-code-end]

### Yanıt

[inline-code-attrs-start title = 'Yorumlar Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Başarısızlık durumunda dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-date' | 'unauthorized-page' | 'invalid-pagination-request' | 'invalid-limit' | 'invalid-limit-children' | 'invalid-skip' | 'invalid-skip-children' | 'invalid-max-tree-depth'
    /** Başarısızlık durumunda dahil edilir. **/
    reason?: string
    /** Yorumlar! **/
    comments: Comment[]
}
[inline-code-end]

### Faydalı İpuçları

#### URL ID

Muhtemelen `Comment` API'sini `urlId` parametresiyle kullanmak istersiniz. Size sunulan `urlId` değerlerinin nasıl göründüğünü görmek için önce `Pages` API'sini çağırabilirsiniz.

#### Anonim İşlemler

Anonim yorum yapma için, yorumları alırken ve işaretleme ile engelleme işlemlerini gerçekleştirirken muhtemelen `anonUserId` geçmek istersiniz.

(!) Bu, birçok uygulama mağazası için gereklidir, çünkü kullanıcılar oturum açmamış olsalar bile görebildikleri kullanıcı tarafından oluşturulmuş içeriği işaretleyebilmelidir. Bunu yapmamak, uygulamanızın söz konusu mağazadan kaldırılmasına neden olabilir.

#### Yorumların Döndürülmemesi

Yorumlarınızın onaylandığını ve spam olmadığını kontrol edin.