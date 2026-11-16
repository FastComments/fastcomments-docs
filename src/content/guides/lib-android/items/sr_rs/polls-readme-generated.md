Анкете раде исто као у веб виџету, и укључују се из истих подешавања виџета у вашем FastComments контролном панелу (или путем `pollCreation` / `pollVoting` изнад).

- Коментари са анкетом приказују је испод текста. Читаоци гласају додиром опције, могу да прикажу или сакрију резултате пре гласања и могу да промене глас. Збиреви се ажурирају уживо како други гласају.
- Анкете могу да се затворе на датум, да сакрију резултате док не гласате, и да учине гласаче видљивим администраторима или свима („Погледај ко је гласао").
- Када гледалац може да креира анкете, поље за унос коментара у `FastCommentsView`-у приказује дугме за анкету које отвара градитељ анкета. Ред траке са алаткама појављује се за њега чак и ако никада нисте укључили траку; `sdk.setCommentToolbarEnabled(false)` је скрива, укључујући и дугме за анкету, а `setPollCreationEnabled(false)` на пољу за унос уклања само дугме за анкету. Позовите `setCommentToolbarEnabled` пре креирања `FastCommentsView`-а, пошто поље за унос чита ову поставку када прими SDK; за приказ који је већ на екрану, уместо тога позовите `getBottomCommentInput().setToolbarVisible(false)`. Поље за унос у живом ћаскању га изоставља, као и веб живо ћаскање.

Да бисте објавили анкету из сопственог корисничког интерфејса, проследите `CommentPollInput` у `postComment`:

```java
CommentPollInput poll = new CommentPollInput()
        .question("Where should we eat?")
        .options(Arrays.asList("Tacos", "Ramen"))
        // Опционо: ISO 8601 датум затварања, вредност PollPrivacy и скривени резултати док не гласате.
        .closesAt(Instant.now().plus(1, ChronoUnit.DAYS).toString())
        .privacy(PollPrivacy.NUMBER_2.getValue())
        .requireVoteToSeeResults(true);
sdk.postComment("Lunch vote", null, null, poll, callback);
```

`sdk.votePoll(commentId, optionId, callback)` и `sdk.getPollVoters(commentId, page, search, callback)` покривају гласање и списак гласача. Боје анкете су наведене у THEMING.md.