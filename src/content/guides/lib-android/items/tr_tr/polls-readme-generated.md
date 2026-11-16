Anketler web widget'ındaki gibi çalışır ve FastComments panonuzda aynı widget ayarlarından açılır (veya yukarıdaki `pollCreation` / `pollVoting` ile).

- Anket içeren yorumlar, anketi metinlerinin altında gösterir. Okuyucular bir seçeneğe dokunarak oy verir, oy vermeden önce sonuçları gösterebilir veya gizleyebilir ve oylarını değiştirebilir. Sayılar, başkaları oy verdikçe canlı olarak güncellenir.
- Anketler bir tarihte kapanabilir, siz oy verene kadar sonuçları gizleyebilir ve oy verenleri yöneticilere veya herkese görünür yapabilir ("Oy verenleri gör").
- Görüntüleyici anket oluşturabiliyorsa, `FastCommentsView`'ın yorum giriş alanı, anket oluşturucuyu açan bir anket düğmesi gösterir. Araç çubuğu satırı, araç çubuğunu hiç açmamış olsanız bile bunun için görünür; `sdk.setCommentToolbarEnabled(false)` bunu gizler (anket düğmesi dahil) ve girişteki `setPollCreationEnabled(false)` yalnızca anket düğmesini kaldırır. `FastCommentsView`'ı oluşturmadan önce `setCommentToolbarEnabled` çağırın, çünkü giriş alanı SDK'yı aldığında bunu okur; ekranda zaten görünen bir görünüm için bunun yerine `getBottomCommentInput().setToolbarVisible(false)` çağırın. Canlı sohbet girişi bunu içermez, tıpkı web canlı sohbeti gibi.

Kendi arayüzünüzden bir anket göndermek için `postComment`'a bir `CommentPollInput` geçirin:

```java
CommentPollInput poll = new CommentPollInput()
        .question("Where should we eat?")
        .options(Arrays.asList("Tacos", "Ramen"))
        // Optional: an ISO 8601 close date, PollPrivacy value, and results hidden until you vote.
        .closesAt(Instant.now().plus(1, ChronoUnit.DAYS).toString())
        .privacy(PollPrivacy.NUMBER_2.getValue())
        .requireVoteToSeeResults(true);
sdk.postComment("Lunch vote", null, null, poll, callback);
```

`sdk.votePoll(commentId, optionId, callback)` ve `sdk.getPollVoters(commentId, page, search, callback)` oy vermeyi ve oy verenler listesini kapsar. Anket renkleri THEMING.md içinde listelenmiştir.