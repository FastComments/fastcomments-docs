[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Bu API, bir kullanıcıya gösterim için yorumları almak amacıyla kullanılır. Örneğin, onaylanmamış veya spam yorumları otomatik olarak filtreler.

### Sayfalama

Sayfalama, performans gereksinimlerine ve kullanım durumuna bağlı olarak iki yoldan biriyle yapılabilir:

1. En Hızlı: **Önceden Hesaplanmış Sayfalama**:
   1. Bu, önceden oluşturulmuş widget'larımızı ve istemcilerimizi kullandığınızda FastComments'in nasıl çalıştığıdır.
   2. "İleri"ye tıklamak yalnızca sayfa sayısını artırır.
   3. Bunu bir anahtar-değer deposundan getiriliyormuş gibi düşünebilirsiniz.
   4. Bu şekilde, `page` parametresini `0`'dan başlayarak ve bir sıralama yönünü `direction` olarak tanımlamanız yeterlidir.
   5. Sayfa boyutları özelleştirme kurallarıyla ayarlanabilir.
2. En Esnek: **Esnek Sayfalama**:
   1. Bu şekilde özel `limit` ve `skip` parametreleri tanımlayabilirsiniz. `page` göndermeyin.
   2. `direction` sıralama yönü de desteklenir.
   3. `limit`, `skip` uygulandıktan sonra döndürülecek toplam sayıdır.
      - Örnek: `sayfa boyutu = 100` ve `page = 2` olduğunda `skip = 200, limit = 100` olarak ayarlayın.
   4. Alt yorumlar (child) yine sayfalama içinde sayılır. Bunu `asTree` seçeneğini kullanarak aşabilirsiniz.
      - Alt yorumları `limitChildren` ve `skipChildren` ile sayfalandırabilirsiniz.
      - Döndürülecek konuların derinliğini `maxTreeDepth` ile sınırlayabilirsiniz.

### Konular (Threads)

1. `Önceden Hesaplanmış Sayfalama` kullanıldığında, yorumlar *sayfa* bazında gruplanır ve konulardaki yorumlar genel sayfayı etkiler.
   1. Bu şekilde, konular `parentId`'ye göre istemci tarafında belirlenebilir.
   2. Örneğin; bir sayfada bir üst düzey yorum ve 29 cevap varsa ve API'de `page=0` ayarlanmışsa — sadece üst düzey yorumu ve 29 çocuğu alırsınız.
   3. [Çoklu sayfaları gösteren örnek resim burada.](https://blog.winricklabs.com/images/fc-pagination02.png)
2. `Esnek Sayfalama` kullanıldığında, bir `parentId` parametresi tanımlayabilirsiniz.
   1. Sadece üst düzey yorumları almak için bunu null olarak ayarlayın.
   2. Konuları görüntülemek için API'yi tekrar çağırın ve `parentId` gönderin.
   3. Yaygın bir çözüm, üst düzey yorumlar için bir API çağrısı yapmak ve ardından her yorumun çocuk yorumlarını almak için paralel API çağrıları yapmaktır.
3. __YENİ Şubat 2023 itibarıyla!__ `&asTree=true` kullanarak ağaç olarak getirin.
   1. Bunu `Ağaç Olarak Esnek Sayfalama` olarak düşünebilirsiniz.
   2. Sadece üst düzey yorumlar sayfalamada sayılır.
   3. Ağacı kökten başlatmak için `parentId=null` ayarlayın (bu parametreyi ayarlamanız gerekir).
   4. Sayfalama için `skip` ve `limit` ayarlayın.
   5. `asTree` değerini `true` yapın.
   6. Bu senaryoda, altyapımızın daha fazla işlem yapması gerektiği için kredi maliyeti `2x` artar.
   7. İstediğiniz şekilde `maxTreeDepth`, `limitChildren` ve `skipChildren` ayarlayın.

### Ağaçlar Açıklaması

`asTree` kullanıldığında, sayfalama hakkında akıl yürütmek zor olabilir. İşte faydalı bir görsel:

<div class="screenshot white-bg">
    <div class="title">Ağaç Sayfalama Diyagramı</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Tree Pagination Diagram" />
</div>

### Yorumları Bir Kullanıcı Bağlamında Getirme

`/comments` API'si, farklı kullanım durumları için iki bağlamda kullanılabilir:

- Kendi istemcinizi oluşturmak için sıralanmış ve bilgi ile etiketlenmiş yorumları döndürmek için.
  - Bu durumda, bir `contextUserId` sorgu parametresi tanımlayın.
- Özel entegrasyonlar için backend'inizden yorumları almak için.
  - Platform, `contextUserId` olmadan buna varsayılan davranır.

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

[inline-code-attrs-start title = 'Kullanıcı Bağlamında Yorumlar — Esnek Sayfalama'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Kullanıcı Bağlamında Yalnızca Üst Düzey Yorumlar için Esnek Sayfalama'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Yorumları Ağaç Olarak Alma

Yorumların, sayfalama yalnızca üst düzey yorumları sayacak şekilde ağaç olarak dönmesi mümkündür.

[inline-code-attrs-start title = 'Kullanıcı Bağlamında Yorumlar — Ağaç Olarak'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Sadece üst düzey yorumları ve bunların hemen çocuklarını mı almak istiyorsunuz? İşte bir yol:

[inline-code-attrs-start title = 'Yorumlar Ağaç Olarak — Maksimum Derinlik'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

Ancak, UI'nizde her yorumda bir "cevapları göster" düğümü gösterip göstermeyeceğinizi bilmeniz gerekebilir. Yorumları ağaç olarak alırken, ilgili olduğunda yorumlara eklenen `hasChildren` özelliği vardır.

### Etiket (Hash Tag) ile Ağaç Olarak Yorum Alma

API ile tüm kiracı (tenant) kapsamında (tek bir sayfa veya `urlId` ile sınırlı olmadan) etiket ile arama yapmak mümkündür.

Bu örnekte `urlId`'yi atlıyoruz ve birden fazla etiket ile arama yapıyoruz. API yalnızca istenen tüm etiketlere sahip yorumları döndürecektir.

[inline-code-attrs-start title = 'Kullanıcı Bağlamında Yorumlar Ağaç Olarak, Etikete Göre'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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
    /** The urlId (page url, or article id) the comments are associated with. **/
    urlId?: string
    /** Limit the comments returned by this user. **/
    userId?: string
    /** Use this to search by hashtag. To drill down to the intersection of multiple hashtags, do &hashTag=a&hashTag=b. **/
    hashTag?: string
    /** The sort direction. Default is MR (Most Relevant). Other options are OF (Oldest First) and NF (Newest First). **/
    direction?: 'MR' | 'OF' | 'NF'
    /** Precalculated Pagination: The page to fetch, starting with 0. Pass -1 for all comments (up to 250). **/
    page?: number
    /** Flexible Pagination: How many comments should we return? **/
    limit?: number
    /** Flexible Pagination: How many child comments should we return for each parent? **/
    limitChildren?: number
    /** Flexible Pagination: How many comments should we skip? **/
    skip?: number
    /** Flexible Pagination: How many child comments should we skip for each parent? **/
    skipChildren?: number
    /** For determining blocked and flagged comments. **/
    contextUserId?: string
    /** For determining blocked and flagged comments. **/
    anonUserId?: string
    /** For fetching child comments. **/
    parentId?: string
    /** For fetching as a tree. **/
    asTree?: boolean
    /** How far into the tree should we return data? 0 returns no children. 1 returns immediate children, etc. **/
    maxTreeDepth?: number
}
[inline-code-end]

### Yanıt

[inline-code-attrs-start title = 'Yorumlar Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-date' | 'unauthorized-page' | 'invalid-pagination-request' | 'invalid-limit' | 'invalid-limit-children' | 'invalid-skip' | 'invalid-skip-children' | 'invalid-max-tree-depth'
    /** Included on failure. **/
    reason?: string
    /** The comments! **/
    comments: Comment[]
}
[inline-code-end]

### Faydalı İpuçları

#### URL ID

Muhtemelen `urlId` parametresiyle `Comment` API'sini kullanmak isteyeceksiniz. Hangi `urlId` değerlerinin size sunulduğunu görmek için önce `Pages` API'sini çağırabilirsiniz.

#### Anonim İşlemler

Anonim yorum için muhtemelen yorumları alırken ve bayraklama (flagging) ve engelleme (blocking) işlemlerini yaparken `anonUserId` göndermek isteyeceksiniz.

(!) Bu, birçok uygulama mağazası için gereklidir çünkü kullanıcılar giriş yapmamış olsalar bile görebildikleri kullanıcı tarafından oluşturulmuş içeriği işaretleyebilmelidir. Bunu yapmamak uygulamanızın ilgili mağazadan kaldırılmasına neden olabilir.

#### Yorumlar Döndürülmüyor

Yorumlarınızın onaylandığını ve spam olmadığını kontrol edin.

---