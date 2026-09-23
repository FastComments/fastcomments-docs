A `Poll` is attached to a comment, rather than being an object of its own. It is created with the comment
(see `POST /api/v1/comments`), or added to an existing comment later with `PUT /api/v1/polls/:commentId`.

The vote counts are kept on the poll itself, so reading a poll gives you the results without having to add
anything up. The individual votes behind those counts are `PollVote` objects.

Each option has an `id` that is generated when the poll is created. That id is what you use to cast a vote,
to relabel an option, and to keep an option (and its votes) when you `PUT` the poll with options added or
removed. It is the only safe way to refer to an option - never its position in the list.

[inline-code-attrs-start title = 'Poll Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPollOption {
    id: string
    label: string
    votes: number
}

interface CommentPoll {
    question: string
    options: CommentPollOption[]
    totalVotes: number
    /** When set and in the past, the poll is closed and no longer accepts votes. **/
    closesAt?: string | null
    /** 0 anonymous (the default), 1 admins and moderators, 2 everyone. Absent means anonymous. **/
    privacy?: 0 | 1 | 2 | null
    /** When true, the counts are hidden from anyone who has not voted yet. Absent means false. **/
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

### Limits

- A question is required, and is at most 200 characters.
- A poll has between 2 and 10 options.
- An option label is required, is at most 100 characters, and must be unique within the poll (ignoring case).
- `closesAt` must be in the future when the poll is created. To close a poll immediately, `PATCH` it with a
  date in the past.

### Site Settings

Polls obey your site configuration, which you can change under Customize Widget:

- Polls must be enabled before a poll can be created, or the API responds with `polls-disabled`.
- Voting can be limited to logged-in users, in which case a vote sent with only an `anonUserId` is rejected
  with `poll-login-required`.
