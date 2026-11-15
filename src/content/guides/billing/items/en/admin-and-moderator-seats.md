On Flex, people who can administer or moderate your account are billed as monthly seats. This is the part of a Flex bill that most often surprises people, usually because seats were created without anyone noticing.

### What Counts As A Seat

- **Admins**: users on your account's `Users` page with any administrative permission (billing, customization, analytics, moderation, and so on). The first admin, normally the account owner, is free; each additional admin is a seat.
- **Moderators**: people on the `Moderators` page who have accepted their invite. A pending invite is not billed. A user whose only permission is comment moderation is billed as a moderator, not an admin.
- **SSO Admins** and **SSO Moderators**: SSO users your site signs in with `isAccountOwner`, `isAdminAdmin`, or `isCommentModeratorAdmin` set in the SSO payload. They are billed at the same rates as regular admins and moderators. See [SSO User Structure](/guide-api.html#sso-user-structure).

Seats are deduplicated by email address, so one person who is both an admin and a moderator, or who has both an SSO user and a dashboard login with the same email, is billed once, at the higher rate. Someone with two different email addresses is two seats.

On accounts billed by monthly active users, an SSO moderator who did not log in during the month is not billed for that month.

### Seats You Did Not Add

Two features create seats automatically:

- **SSO admin sync.** When an SSO user signs in with an email that matches one of your dashboard users, that user's admin or moderator permissions are copied to the SSO user. With WordPress SSO this means every WordPress administrator can become a FastComments admin or moderator. Turn it off under `Customize` with the `Disable Automatic Admin Migration` option on your customization rule.
- **Different emails for the same person.** If your dashboard login uses one email and your SSO user another, they are two seats. Using the same email on both merges them.

### Checking And Removing Seats

The [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) page lists your `Moderators`, `SSO Admins`, and `SSO Moderators` counts, and the next invoice preview shows what they cost. Remove people you do not need from the `Users` and `Moderators` pages, or drop the admin flags from your SSO payload. Seats are counted when the invoice is created at the end of the month, so a seat removed before then is not billed for that month.

See [Adding Moderators](/guide-users.html#adding-moderators) and [Updating Administrator Permissions](/guide-users.html#updating-administrator-permissions) for how to manage both.
