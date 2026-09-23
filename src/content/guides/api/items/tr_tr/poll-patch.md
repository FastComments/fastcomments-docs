[api-resource-header-start name = 'Poll'; route = 'PATCH /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Bir anketi oylarını etkilemeden düzenler. Bunu sorudaki veya bir seçenekteki yazım hatasını düzeltmek, anketi kapatmak veya yeniden açmak,
ya da kimin oy kullandığını görebileceğini değiştirmek için kullanın.

Seçenekler `id`'leriyle adreslenir ve bir `PATCH` adını verdiğiniz seçeneklerin etiketlerini değiştirir. Seçenek eklemek, kaldırmak veya yeniden sıralamak için,
tam seçenek listesini `PUT /api/v1/polls/:commentId` adresine gönderin: `id`'leriyle gönderdiğiniz seçenekler oylarını da korur.

Her alan isteğe bağlıdır, ancak en az bir tanesi sağlanmalıdır.

[inline-code-attrs-start title = 'Anket Yaması cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "Which release should we cut first?",
	"options": [{"id": "the-option-id", "label": "The bugfix release"}]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Şimdi Bir Anketi Kapat cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"closesAt": "2020-01-01T00:00:00.000Z"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Anket Yaması İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPatchQueryParams {
    tenantId: string
    API_KEY: string
}

interface PollPatchBody {
    question?: string | null
    /** Relabels existing options. Every id given must already be on the poll. **/
    options?: { id: string, label: string }[] | null
    /** A date in the past closes the poll now. null reopens a closed poll. **/
    closesAt?: string | null
    /** 0 anonymous, 1 admins and moderators, 2 everyone. **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = 'Anket Yaması Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'poll-not-found' | 'poll-invalid' | 'poll-privacy-locked' | 'locked'
    /** Included on failure. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### Diğer Notlar

- Ankette bulunmayan bir seçenek kimliği adlandırmak, sessizce hiçbir şey yapmaktan ziyade `poll-invalid` hatası verir.
- Etiketler, değiştirmediğiniz seçenekleri de sayarak, anket içinde benzersiz kalmalıdır.
- Anket oluştururken farklı olarak, burada `closesAt` geçmişte olabilir – bu, bir anketi hemen kapatmanın yoludur.
- Anket gizliliği oylar alındıktan sonra daraltılabilir ancak genişletilemez.
- Kilitli bir yorumun anketi değiştirilemez ve `locked` hatası verir.