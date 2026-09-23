[api-resource-header-start name = 'Poll'; route = 'PUT /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Mevcut bir yoruma anket ekler veya zaten sahip olduğu anketin tam durumunu ayarlar.

Gövde, tam anketi içerir ve gönderdiğiniz seçenekler, o sırayla anketin seçenekleri olur. Her seçenek, `id`'siyle eşleştirilir:

- Mevcut bir seçeneğin `id`'siyle gönderilen bir seçenek, o seçeneği ve oylarını korur. Etiketi ve konumu, gönderdiğiniz şekilde güncellenir.
- `id` olmadan gönderilen bir seçenek eklenir ve oyları yoktur.
- Bıraktığınız mevcut bir seçenek, üzerindeki oylarla birlikte kaldırılır. `totalVotes` aynı miktarda azalır.

Bu yüzden bir seçenek eklemek için, mevcut seçenekleri id'leriyle birlikte yeni seçeneği id'siz gönderin. Bir seçeneği kaldırmak için, listeden onu çıkartarak gönderin. Seçenek id'leri, `GET /api/v1/polls/:commentId` isteğiyle dönen ankette bulunur.

Hiç id göndermemek, tüm seçenekleri değiştirir ve ankete daha önce verilen tüm oyları siler. Anketin oyları varsa, bu `replaceVotes=true` gerektirir; aksi takdirde API `replace-votes-required` yanıtını verir.

Diğer alanlar da değiştirilir: `closesAt`, `privacy` veya `requireVoteToSeeResults` bırakılırsa, varsayılan değerine sıfırlanır. Tek bir alanı değiştirmek ve diğerlerini olduğu gibi bırakmak için `PATCH /api/v1/polls/:commentId` kullanın.

[inline-code-attrs-start title = 'Anket PUT cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PUT \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "Which release should we cut first?",
	"options": [
		{"id": "existing-option-id", "label": "The bugfix release"},
		{"label": "The feature release"}
	],
	"closesAt": "2026-12-31T00:00:00.000Z"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Anket PUT İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutQueryParams {
    tenantId: string
    API_KEY: string
    /** Required to keep none of the existing option ids when the poll has votes, since that deletes them all. **/
    replaceVotes?: boolean
}

interface PollPutOption {
    /** The id of an existing option, to keep it and its votes. Omit to add a new option. **/
    id?: string | null
    label: string
}

interface PollPutBody {
    question: string
    /** The complete, ordered list. Existing options left out are removed with their votes. **/
    options: PollPutOption[]
    /** Must be in the future when the comment has no poll yet. Omit for a poll that stays open. **/
    closesAt?: string | null
    /** 0 anonymous (the default), 1 admins and moderators, 2 everyone. **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = 'Anket PUT Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'polls-disabled' | 'poll-invalid' | 'replace-votes-required' | 'poll-privacy-locked' | 'locked'
    /** Included on failure. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### Diğer Notlar

- `poll` içinde olmayan bir `id` ya da aynı `id` iki kez verilirse, `poll-invalid` hatası oluşur. Anketi olmayan bir yorumda henüz seçenek id'leri yoktur, bu yüzden gönderilen her seçenek `id`'yi boş bırakmalıdır.
- Anket gizliliği oylar alındıktan sonra daraltılabilir ancak genişletilemez.
- Bu API site ayarlarınızı uygular. Site veya sayfa için anketler etkinleştirilmemişse, `polls-disabled` hatası döner.
- Kilitli bir yorumun anketi değiştirilemez ve `locked` hatası verir.
- Bağlı widget'lar canlı olarak güncellenir, böylece izleyiciler yeni anketi sayfayı yeniden yüklemeden görür.