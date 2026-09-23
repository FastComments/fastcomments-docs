A `PollVote` is one person's answer to a poll. The counts shown on the poll itself are kept in step with these,
so you only need these when you want to know *who* voted for what, rather than the totals.

A voter has at most one vote per poll. Voting again moves their existing vote to the new option instead of
adding a second one, and `updatedAt` records when that happened.

`voterId` is the `userId` when the voter was logged in, and the `anonUserId` otherwise.

[inline-code-attrs-start title = 'PollVote Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVote {
    id: string
    tenantId: string
    commentId: string
    urlId: string
    /** The userId when the voter was logged in, otherwise the anonUserId. **/
    voterId: string
    optionId: string
    createdAt: string
    /** When the voter last moved their vote to a different option. **/
    updatedAt?: string
}
[inline-code-end]

### Privacy

The poll's `privacy` setting applies to this API the same way it applies in the comment widget:

- **Anonymous** (the default): nobody can see how anyone voted, so the votes cannot be read.
  `GET /api/v1/poll-votes` and `GET /api/v1/poll-votes/:id` respond with `poll-anonymous`. The poll's counts
  are still available from `GET /api/v1/polls/:commentId`.
- **Admins and moderators**: your API key belongs to your site's admin, so it can read the votes.
- **Everyone**: the votes can be read.

Poll privacy can be narrowed but not widened once it has votes.
