[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Bu API, bir kullanıcıya göstermek üzere yorumları almak için kullanılır. Örneğin, otomatik olarak onaylanmamış veya istenmeyen (spam) yorumları filtreler.

### Pagination

Sayfalama, performans gereksinimlerine ve kullanım durumuna bağlı olarak iki yoldan biriyle yapılabilir:

1. En hızlı: **Precalculated Pagination**:
   1. Bu, önceden oluşturulmuş widget ve istemcilerimizi kullandığınızda FastComments'ın nasıl çalıştığıdır.
   2. “next”e tıklamak yalnızca sayfa sayacını artırır.
   3. Bunu bir anahtar-değer deposundan alınmış gibi düşünebilirsiniz.
   4. Bu şekilde, `0`'dan başlayan bir `page` parametresi ve `direction` olarak bir sıralama yönü tanımlamanız yeterlidir.
   5. Sayfa boyutları özelleştirme kurallarıyla ayarlanabilir.
2. En esnek: **Flexible Pagination**:
   1. Bu şekilde özel `limit` ve `skip` parametreleri tanımlayabilirsiniz. `page` göndermeyin.
   2. `direction` sıralaması da desteklenir.
   3. `limit`, `skip` uygulandıktan sonra döndürülecek toplam sayıyı belirtir.
      - Örnek: `page size = 100` ve `page = 2` iken `skip = 200, limit = 100` ayarlayın.
   4. Alt yorumlar hâlâ sayfalamada sayılır. Bunu `asTree` seçeneğiyle aşabilirsiniz.
      - `limitChildren` ve `skipChildren` ile alt yorumları sayfalayabilirsiniz.
      - `maxTreeDepth` ile döndürülen konu derinliğini sınırlayabilirsiniz.

### Threads

1. `Precalculated Pagination` kullanıldığında, yorumlar *sayfa* bazında gruplanır ve konu içindeki yorumlar genel sayfayı etkiler.
   1. Bu şekilde, konular istemci tarafında `parentId` temel alınarak belirlenebilir.
   2. Örneğin, bir üst düzey yorum ve 29 yanıt içeren bir sayfada API'de `page=0` ayarlandığında yalnızca üst düzey yorum ve 29 alt yorum alınır.
2. `Flexible Pagination` kullanıldığında bir `parentId` parametresi tanımlayabilirsiniz.
   1. Bunu `null` olarak ayarlarsanız yalnızca üst düzey yorumları alırsınız.
   2. Ardından konuları görüntülemek için API'yi tekrar çağırıp `parentId` gönderin.
   3. Yaygın bir çözüm, üst düzey yorumlar için bir API çağrısı yapıp ardından her yorumun alt yorumları için paralel API çağrıları yapmaktır.
3. __NEW As of Feb 2023!__ `&asTree=true` kullanarak ağaç olarak alın.
   1. Bunu `Flexible Pagination as a Tree` olarak düşünebilirsiniz.
   2. Yalnızca üst düzey yorumlar sayfalamada sayılır.
   3. `parentId=null` ayarlayarak ağacı kökten başlatın (`parentId` ayarlamanız gerekir).
   4. Sayfalama için `skip` ve `limit` ayarlayın.
   5. `asTree` değerini `true` yapın.
   6. Krediler maliyeti `2x` artar, çünkü bu senaryoda arka uç çok daha fazla iş yapmalıdır.
   7. `maxTreeDepth`, `limitChildren` ve `skipChildren` değerlerini istediğiniz gibi ayarlayın.

### Trees Explained

`asTree` kullanıldığında, sayfalama hakkında mantık yürütmek zor olabilir. İşte kullanışlı bir grafik:

<div class="screenshot white-bg">
    <div class="title">Ağaç Sayfalama Diyagramı</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Ağaç Sayfalama Diyagramı" />
</div>

### Fetching Comments in The Context of a User

`/comments` API'si iki bağlamda, farklı kullanım durumları için kullanılabilir:

- Kendi istemcinizi oluşturmak için sıralanmış ve etiketlenmiş yorumları döndürmek.
  - Bu durumda bir `contextUserId` sorgu parametresi tanımlayın.
- Özel entegrasyonlar için yorumları arka uçtan almak.
  - Platform, `contextUserId` olmadan buna varsayılan olarak geçer.

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

[inline-code-attrs-start title = 'Yorumlar Kullanıcı Bağlamında Esnek Sayfalama'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Yorumlar Kullanıcı Bağlamında Yalnızca Üst Düzey Yorumlar İçin Esnek Sayfalama'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Get Comments as a Tree

Yorumları bir ağaç olarak alabilir ve sayfalama yalnızca üst düzey yorumları sayar.

[inline-code-attrs-start title = 'Yorumlar Ağaç Şeklinde Kullanıcı Bağlamında'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Yalnızca üst düzey yorumları ve doğrudan alt yorumları almak ister misiniz? İşte bir yol:

[inline-code-attrs-start title = 'Yorumlar Ağaç Şeklinde Maksimum Derinlik ile'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

Ancak UI'nizde her yorumun yanıtları göster düğmesini gösterip göstermeyeceğinizi bilmeniz gerekebilir. Ağaç üzerinden yorum alırken, uygulanabilir olduğunda yorumlara `hasChildren` özelliği eklenir.

### Get Comments as a Tree, Searching by Hash Tag

API'yi kullanarak tüm tenantınızda (tek bir sayfaya veya `urlId`'ye sınırlı olmadan) hashtag ile arama yapabilirsiniz.

Bu örnekte `urlId`'yi atlıyoruz ve birden fazla hashtag ile arama yapıyoruz. API yalnızca istenen tüm hashtag'lere sahip yorumları döndürür.

[inline-code-attrs-start title = 'Yorumlar Ağaç Şeklinde Kullanıcı Bağlamında, Hashtag ile'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### All Request Params

[inline-code-attrs-start title = 'Yorumlar İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Yorumların ilişkilendirildiği urlId (sayfa URL'si veya makale kimliği). **/
    urlId?: string
    /** Bu kullanıcı tarafından döndürülen yorumları sınırlayın. **/
    userId?: string
    /** Hashtag ile arama yapmak için bunu kullanın. Birden fazla hashtag'in kesişimini bulmak için &hashTag=a&hashTag=b şeklinde kullanın. **/
    hashTag?: string
    /** Sıralama yönü. Varsayılan MR (En İlgili). Diğer seçenekler OF (En Eskisi İlk) ve NF (En Yeni İlk). **/
    direction?: 'MR' | 'OF' | 'NF'
    /** Önceden Hesaplanmış Sayfalama: Alınacak sayfa, 0'dan başlar. Tüm yorumlar için -1 gönderin (250'ye kadar). **/
    page?: number
    /** Esnek Sayfalama: Kaç yorum döndürmeliyiz? **/
    limit?: number
    /** Esnek Sayfalama: Her ebeveyn için kaç alt yorum döndürmeliyiz? **/
    limitChildren?: number
    /** Esnek Sayfalama: Kaç yorumu atlamalıyız? **/
    skip?: number
    /** Esnek Sayfalama: Her ebeveyn için kaç alt yorumu atlamalıyız? **/
    skipChildren?: number
    /** Engellenen ve işaretlenmiş yorumları belirlemek için. **/
    contextUserId?: string
    /** Engellenen ve işaretlenmiş yorumları belirlemek için. **/
    anonUserId?: string
    /** Alt yorumları almak için. **/
    parentId?: string
    /** Ağaç olarak almak için. **/
    asTree?: boolean
    /** Ağaçta ne kadar derine veri döndürmeliyiz? 0 hiçbir alt yorum döndürmez. 1 doğrudan alt yorumları döndürür, vb. **/
    maxTreeDepth?: number
}
[inline-code-end]

### The Response

[inline-code-attrs-start title = 'Yorumlar Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Başarısızlıkta dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-date' | 'unauthorized-page' | 'invalid-pagination-request' | 'invalid-limit' | 'invalid-limit-children' | 'invalid-skip' | 'invalid-skip-children' | 'invalid-max-tree-depth'
    /** Başarısızlıkta dahil edilir. **/
    reason?: string
    /** Yorumlar! **/
    comments: Comment[]
}
[inline-code-end]

### Helpful Tips

#### URL ID

`Comment` API'sini `urlId` parametresiyle kullanmak isteyebilirsiniz. Önce `Pages` API'sini çağırarak size mevcut olan `urlId` değerlerinin nasıl göründüğünü görebilirsiniz. 

#### Anonymous Actions

Anonim yorumlama için yorumları alırken ve işaretleme ve engelleme işlemleri yaparken `anonUserId` göndermeniz muhtemeldir.

(!) Bu, birçok uygulama mağazası için gereklidir; kullanıcılar oturum açmasalar bile görebildikleri kullanıcı tarafından oluşturulan içeriği işaretleyebilmelidir. Bunu yapmazsanız uygulamanız ilgili mağazadan kaldırılabilir.

#### Comments Not Being Returned

Yorumlarınızın onaylandığını ve istenmeyen (spam) olmadığını kontrol edin.

---