A `PollVote` bir kişinin ankete verdiği cevaptır. Anketin kendisinde gösterilen sayımlar bu verilerle senkronizedir, bu yüzden toplamları değil, *kim* neye oy verdiğini bilmek istediğinizde bu verilere ihtiyacınız olur.

Bir seçmenin bir ankette en fazla bir oyu vardır. Tekrar oy vermek, ikinci bir oy eklemek yerine mevcut oyunu yeni seçeneğe taşır ve `updatedAt` bu olayın ne zaman gerçekleştiğini kaydeder.

`voterId` seçmen oturum açtığında `userId`'dir, aksi takdirde `anonUserId`'dir.

[inline-code-attrs-start title = 'PollVote Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVote {
    id: string
    tenantId: string
    commentId: string
    urlId: string
    /** Seçmen oturum açtığında userId, aksi takdirde anonUserId. **/
    voterId: string
    optionId: string
    createdAt: string
    /** Seçmenin oyunu en son farklı bir seçeneğe taşıdığı zaman. **/
    updatedAt?: string
}
[inline-code-end]

### Gizlilik

Anketin `privacy` ayarı, yorum widget'ında olduğu gibi bu API'ye de aynı şekilde uygulanır:

- **Anonymous** (varsayılan): kimsenin birinin nasıl oy verdiğini göremez, bu yüzden oylar okunamaz. `GET /api/v1/poll-votes` ve `GET /api/v1/poll-votes/:id` `poll-anonymous` ile yanıt verir. Anketin sayımları hâlâ `GET /api/v1/polls/:commentId` üzerinden erişilebilir.
- **Admins and moderators**: API anahtarınız sitenizin yöneticisine aittir, bu yüzden oyları okuyabilir.
- **Everyone**: oylar okunabilir.

Anket gizliliği oylar alındıktan sonra daraltılabilir ancak genişletilemez.