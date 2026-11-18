Polls work the same as in the web widget, and turn on from the same widget settings in your FastComments dashboard: `pollCreation` (who can attach a poll: disabled, admins and moderators, or everyone) and `pollVoting` (logged in users or anyone).

- Comments with a poll show it under their text. Readers vote by tapping an option, can show or hide results before voting, and can change their vote. Tallies update live as others vote.
- Polls can close on a date, hide results until you vote, and make voters visible to admins or to everyone ("See who voted").
- When the viewer may create polls, `FastCommentsView`'s comment input shows a poll button that opens the poll builder. A poll is posted with a comment, so the comment text is required. The live chat input leaves the button out, like the web live chat.

Voting needs a logged-in user, so use [SSO](#authentication-sso-readme-generated). The server ties an anonymous vote to a session cookie, which the API client doesn't keep, so with `pollVoting` set to anyone, a viewer with no SSO user sees "Log in to vote". Polls that hide results until you vote don't get live tally updates; those voters see new numbers on their next load.

To post a poll from your own UI, pass a `CommentPollInput` to `postComment`:

```swift
let draft = PollDraft(question: "Where should we eat?", options: ["Tacos", "Ramen"], closesDays: 1)
if draft.validate() == nil {
    try await sdk.postComment(text: "Lunch vote", poll: draft.toInput())
}
```

`sdk.votePoll(commentId:optionId:)` and `sdk.getPollVoters(commentId:page:search:)` cover voting and the voter list. Poll colors are listed under [Poll Colors](#theming-readme-generated).