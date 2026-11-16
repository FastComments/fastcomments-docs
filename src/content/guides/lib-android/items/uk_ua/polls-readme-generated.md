Опитування працюють так само, як у веб-віджеті, і вмикаються з тих самих налаштувань віджета у вашій панелі FastComments (або за допомогою `pollCreation` / `pollVoting` вище).

- Коментарі з опитуванням показують його під текстом. Читачі голосують, натискаючи на варіант, можуть показати або приховати результати перед голосуванням і можуть змінити свій голос. Підрахунки оновлюються в реальному часі, коли голосують інші.
- Опитування можуть закриватися за датою, приховувати результати до голосування та робити учасників голосування видимими для адміністраторів або для всіх («Побачити, хто проголосував»).
- Коли глядач може створювати опитування, поле введення коментаря у `FastCommentsView` показує кнопку опитування, яка відкриває конструктор опитувань. Рядок панелі інструментів з'являється для неї, навіть якщо ви ніколи не вмикали панель інструментів; `sdk.setCommentToolbarEnabled(false)` приховує її, включно з кнопкою опитування, а `setPollCreationEnabled(false)` на полі введення прибирає лише кнопку опитування. Викличте `setCommentToolbarEnabled` перед створенням `FastCommentsView`, оскільки поле введення зчитує це значення, коли отримує SDK; для перегляду, який уже на екрані, натомість викличте `getBottomCommentInput().setToolbarVisible(false)`. Поле введення живого чату не показує її, як і веб-версія живого чату.

Щоб опублікувати опитування з власного інтерфейсу, передайте `CommentPollInput` до `postComment`:

```java
CommentPollInput poll = new CommentPollInput()
        .question("Where should we eat?")
        .options(Arrays.asList("Tacos", "Ramen"))
        // Необов'язково: дата закриття в форматі ISO 8601, значення PollPrivacy та приховані результати до голосування.
        .closesAt(Instant.now().plus(1, ChronoUnit.DAYS).toString())
        .privacy(PollPrivacy.NUMBER_2.getValue())
        .requireVoteToSeeResults(true);
sdk.postComment("Lunch vote", null, null, poll, callback);
```

`sdk.votePoll(commentId, optionId, callback)` та `sdk.getPollVoters(commentId, page, search, callback)` охоплюють голосування та список тих, хто проголосував. Кольори опитувань перелічені в THEMING.md.