Billing is restricted to two kinds of users:

- The `Account Owner`, the user who created the account.
- Any user with the `Billing Admin` permission.

They are the only users who can change the plan, save or replace a card, open `Invoice Settings`, set Flex spend limits, and add credit. Anyone else who tries sees `Only the account owner or a billing admin can change the plan or payment method.`

Billing emails, such as the mid-trial reminder, failed payment warnings, and deactivation notices, are sent to users with the `Billing Admin` permission. Give the permission to whoever should receive them, for example someone in finance, and they will get the emails without needing any other access to the dashboard. The reminders sent after a trial has ended without billing info go to every dashboard user on the account. Each of these emails has an unsubscribe link; a user who opts out of account notifications stops receiving them.

Invoices themselves go to the `Billing Email` in [Invoice Settings](#invoice-settings), which does not have to belong to a dashboard user at all.

The [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) page needs the `Analytics Admin` permission instead, since it shows usage rather than payment details.

To grant or remove the permission, open the `Users` page and edit the user. See [Updating Administrator Permissions](/guide-users.html#updating-administrator-permissions). On Flex, remember that each additional admin is a billed seat, see [Admin and Moderator Seats](#admin-and-moderator-seats).

If you are logged in and cannot see `Billing Info` at all, or it sends you back to the home page, you are logged in as a commenter account rather than a dashboard user. Sign out and sign in with the account that owns the site.
