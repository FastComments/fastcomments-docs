Polls can also be managed from your own code with the REST API, for example to add a poll to a comment from your backend, close a poll, or export the votes:

- [Poll Structure](/guide-api.html#poll-structure) anket nesnesini ve limitlerini açıklar.
- [GET](/guide-api.html#poll-get), [PUT](/guide-api.html#poll-put), [PATCH](/guide-api.html#poll-patch), and [DELETE](/guide-api.html#poll-delete) `/api/v1/polls/:commentId` bir yorum üzerindeki anketi okur, değiştirir, düzenler ve kaldırır.
- [PollVote Structure](/guide-api.html#poll-vote-structure) tek bir oyu açıklar.
- [GET](/guide-api.html#poll-votes-get), [POST](/guide-api.html#poll-vote-post), and [DELETE](/guide-api.html#poll-vote-delete) `/api/v1/poll-votes` oyları listeler, verir ve kaldırır.

Bir anket, `poll` parametresini `POST /api/v1/comments` isteğine gönderilerek yorumuyla birlikte de oluşturulabilir.

API, widget ile aynı ayarları izler: anketlerin etkinleştirilmiş olması gerekir ve oturum açmış bir kullanıcı olmadan verilen oylar, anonim anket oylarına izin verilmedikçe reddedilir. Anonim anketlerdeki bireysel oylar API üzerinden okunamaz.