Polls work the same as in the web widget, and turn on from the same widget settings in your FastComments dashboard (or with `pollCreation` / `pollVoting` above).

- Comments with a poll show it under their text. Readers vote by tapping an option, can show or hide results before voting, and can change their vote. Tallies update live as others vote.
- Polls can close on a date, hide results until you vote, and make voters visible to admins or to everyone ("See who voted").
- When the viewer may create polls, `FastCommentsView`'s comment input shows a poll button that opens the poll builder. The toolbar row appears for it even if you never turned the toolbar on; `sdk.setCommentToolbarEnabled(false)` hides it, poll button included, and `setPollCreationEnabled(false)` on the input removes only the poll button. Call `setCommentToolbarEnabled` before creating the `FastCommentsView`, since the input reads it when it receives the SDK; for a view already on screen, call `getBottomCommentInput().setToolbarVisible(false)` instead. The live chat input leaves it out, like the web live chat.

To post a poll from your own UI, pass a `CommentPollInput` to `postComment`:

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

`sdk.votePoll(commentId, optionId, callback)` and `sdk.getPollVoters(commentId, page, search, callback)` cover voting and the voter list. Poll colors are listed in THEMING.md.