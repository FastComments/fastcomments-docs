Анкетите работят по същия начин като в уеб уиджета и се включват от същите настройки на уиджета в дашборда на FastComments (или с `pollCreation` / `pollVoting` по-горе).

- Коментарите с анкета я показват под текста си. Читателите гласуват, като докоснат опция, могат да покажат или скрият резултатите преди гласуване и могат да променят гласа си. Резултатите се обновяват на живо, докато други гласуват.
- Анкетите могат да се затворят на определена дата, да скриват резултатите, докато не гласувате, и да правят гласувалите видими за администраторите или за всички („Виж кой гласува").
- Когато зрителят може да създава анкети, полето за коментар на `FastCommentsView` показва бутон за анкета, който отваря конструктора на анкети. Редът с лентата с инструменти се появява за него, дори да не сте включили лентата; `sdk.setCommentToolbarEnabled(false)` я скрива, включително бутона за анкета, а `setPollCreationEnabled(false)` върху полето за въвеждане премахва само бутона за анкета. Извикайте `setCommentToolbarEnabled` преди създаването на `FastCommentsView`, тъй като полето за въвеждане го прочита, когато получи SDK; за изглед, който вече е на екрана, извикайте вместо това `getBottomCommentInput().setToolbarVisible(false)`. Полето за въвеждане на живия чат го пропуска, както уеб чатът на живо.

За да публикувате анкета от собствения си интерфейс, подайте `CommentPollInput` на `postComment`:

```java
CommentPollInput poll = new CommentPollInput()
        .question("Where should we eat?")
        .options(Arrays.asList("Tacos", "Ramen"))
        // По избор: ISO 8601 дата на затваряне, стойност на PollPrivacy и скрити резултати, докато не гласувате.
        .closesAt(Instant.now().plus(1, ChronoUnit.DAYS).toString())
        .privacy(PollPrivacy.NUMBER_2.getValue())
        .requireVoteToSeeResults(true);
sdk.postComment("Lunch vote", null, null, poll, callback);
```

`sdk.votePoll(commentId, optionId, callback)` и `sdk.getPollVoters(commentId, page, search, callback)` покриват гласуването и списъка с гласували. Цветовете на анкетите са изброени в THEMING.md.