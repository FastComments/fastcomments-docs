Anketler web widget'ındaki gibi çalışır ve FastComments kontrol panelinizdeki aynı widget ayarlarından açılır: `pollCreation` (kimler anket ekleyebilir: devre dışı, yöneticiler ve moderatörler veya herkes) ve `pollVoting` (oturum açmış kullanıcılar veya herkes).

- Anket içeren yorumlar anketi metinlerinin altında gösterir. Okuyucular bir seçeneğe dokunarak oy verir, oy vermeden önce sonuçları gösterebilir veya gizleyebilir ve oylarını değiştirebilir. Sayılar, başkaları oy verdikçe canlı olarak güncellenir.
- Anketler bir tarihte kapanabilir, oy verene kadar sonuçları gizleyebilir ve oy verenleri yöneticilere veya herkese görünür yapabilir ("Kimin oy verdiğini gör").
- Görüntüleyici anket oluşturabiliyorsa, `FastCommentsView`'ın yorum giriş alanı anket oluşturucuyu açan bir anket düğmesi gösterir. Anket bir yorumla birlikte gönderilir, bu nedenle yorum metni gereklidir. Canlı sohbet giriş alanı, web canlı sohbetindeki gibi bu düğmeyi içermez.

Oy vermek oturum açmış bir kullanıcı gerektirir, bu nedenle [SSO](#authentication-sso-readme-generated) kullanın. Sunucu anonim bir oyu oturum çerezine bağlar ve API istemcisi bu çerezi tutmaz; dolayısıyla `pollVoting` herkese ayarlandığında bile, SSO kullanıcısı olmayan bir görüntüleyici "Oy vermek için giriş yapın" görür. Oy verene kadar sonuçları gizleyen anketler canlı sayı güncellemesi almaz; bu seçmenler yeni sayıları bir sonraki yüklemede görür.

Kendi arayüzünüzden bir anket göndermek için `postComment`'a bir `CommentPollInput` iletin:

```swift
let draft = PollDraft(question: "Where should we eat?", options: ["Tacos", "Ramen"], closesDays: 1)
if draft.validate() == nil {
    try await sdk.postComment(text: "Lunch vote", poll: draft.toInput())
}
```

`sdk.votePoll(commentId:optionId:)` ve `sdk.getPollVoters(commentId:page:search:)` oy vermeyi ve seçmen listesini kapsar. Anket renkleri [Poll Colors](#poll-colors-readme-generated) altında listelenmiştir.