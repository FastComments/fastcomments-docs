Опросы работают так же, как в веб-виджете, и включаются в тех же настройках виджета в вашей панели FastComments (или с помощью `pollCreation` / `pollVoting` выше).

- Комментарии с опросом показывают его под текстом. Читатели голосуют, нажимая на вариант, могут показать или скрыть результаты до голосования и могут изменить свой голос. Результаты обновляются в реальном времени по мере голосования других участников.
- Опросы могут закрываться по дате, скрывать результаты до голосования и делать голосующих видимыми для администраторов или для всех («Показать, кто голосовал»).
- Когда зритель может создавать опросы, поле ввода комментария в `FastCommentsView` показывает кнопку опроса, открывающую конструктор опросов. Строка панели инструментов появляется для неё, даже если вы никогда не включали панель инструментов; `sdk.setCommentToolbarEnabled(false)` скрывает её, включая кнопку опроса, а `setPollCreationEnabled(false)` на поле ввода удаляет только кнопку опроса. Вызывайте `setCommentToolbarEnabled` до создания `FastCommentsView`, так как поле ввода читает этот параметр при получении SDK; для уже отображаемого представления вызовите вместо этого `getBottomCommentInput().setToolbarVisible(false)`. Поле ввода живого чата её не показывает, как и веб-версия живого чата.

Чтобы опубликовать опрос из собственного интерфейса, передайте `CommentPollInput` в `postComment`:

```java
CommentPollInput poll = new CommentPollInput()
        .question("Where should we eat?")
        .options(Arrays.asList("Tacos", "Ramen"))
        // Необязательно: дата закрытия в формате ISO 8601, значение PollPrivacy и скрытие результатов до голосования.
        .closesAt(Instant.now().plus(1, ChronoUnit.DAYS).toString())
        .privacy(PollPrivacy.NUMBER_2.getValue())
        .requireVoteToSeeResults(true);
sdk.postComment("Lunch vote", null, null, poll, callback);
```

`sdk.votePoll(commentId, optionId, callback)` и `sdk.getPollVoters(commentId, page, search, callback)` покрывают голосование и список голосовавших. Цвета опросов перечислены в THEMING.md.