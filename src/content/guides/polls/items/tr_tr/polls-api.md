Anketler REST API aracılığıyla kendi kodunuzdan da yönetilebilir; örneğin arka ucunuzdan bir yoruma anket eklemek, bir anketi kapatmak veya oyları dışa aktarmak için:

- [Poll Structure](/guide-api.html#poll-structure), anket nesnesini ve sınırlarını açıklar.
- [GET](/guide-api.html#poll-get), [PUT](/guide-api.html#poll-put), [PATCH](/guide-api.html#poll-patch) ve [DELETE](/guide-api.html#poll-delete) `/api/v1/polls/:commentId`, bir yorumdaki anketi okur, değiştirir, düzenler ve kaldırır.
- [PollVote Structure](/guide-api.html#poll-vote-structure), tek bir oyu açıklar.
- [GET](/guide-api.html#poll-votes-get), [POST](/guide-api.html#poll-vote-post) ve [DELETE](/guide-api.html#poll-vote-delete) `/api/v1/poll-votes`, oyları listeler, kullanır ve kaldırır.

Bir anket, `POST /api/v1/comments` isteğine `poll` parametresi geçirilerek yorumuyla birlikte de oluşturulabilir.

API, widget ile aynı ayarları takip eder: anketlerin etkin olması gerekir ve anonim anket oylarına izin verilmedikçe, oturum açmamış kullanıcıların oyları reddedilir. Anonim anketlerdeki bireysel oylar API üzerinden okunamaz.