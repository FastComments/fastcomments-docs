The SDK provides many configuration options through the `CommentWidgetConfig` class:

| Option | Description |
|--------|-------------|
| `tenantId` | Your FastComments account ID |
| `urlId` | ID representing the current page |
| `sso` | SSO token for authentication |
| `allowAnon` | Allow anonymous commenting |
| `voteStyle` | UpDown or Heart voting style |
| `hideAvatars` | Hide user avatars |
| `hasDarkBackground` | Indicates dark mode |
| `customCSS` | Custom CSS styles |
| `enableInfiniteScrolling` | Enable infinite scroll pagination |
| `readonly` | Disable commenting but show comments |
| `disableVoting` | Disable voting functionality |
| `disableLiveCommenting` | Disable real-time updates |
| `pollCreation` | Who can attach a poll to a comment: `NUMBER_0` disabled, `NUMBER_1` admins and moderators, `NUMBER_2` everyone |
| `pollVoting` | Who can vote on polls: `NUMBER_0` logged in users, `NUMBER_1` anyone |