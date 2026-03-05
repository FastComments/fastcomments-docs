When emails sent by FastComments bounce or are marked as spam by the recipient, the email provider adds that address to a
suppression list. FastComments syncs these suppression lists daily so that further emails are not sent to addresses that
cannot receive them.

Users and moderators with suppressed email addresses will not receive any email notifications, including reply notifications,
mention notifications, admin alerts, and digest emails. A red "Email Suppressed" badge will appear next to affected users
and moderators in the admin UI.

#### Viewing Suppressed Emails

Tenant administrators with Manage Data permission can view suppressed emails on the
[Suppressed Emails](https://fastcomments.com/auth/my-account/suppressed-emails) page, under Manage Data.

The page shows a table of all suppressed email addresses associated with your tenant's users, moderators, and commenters.
You can filter by email address using the search field.

#### Removing a Suppression

To remove a suppression, click the **Remove** button next to the entry in the table. You will be taken to a confirmation
page showing the details of the suppression. Click **Confirm Removal** to proceed.

When a suppression is removed, FastComments contacts the email provider to unblock the address and clears the suppression
flag from all associated user and moderator records.

#### Rate Limits

To prevent abuse, removals are rate limited:

- Each email address can only be unsuppressed once every 30 days.
- Each tenant can perform up to 5 removals per calendar month.

If a suppression reappears after removal, it means the email address bounced or was reported as spam again. In that case,
the underlying deliverability issue should be resolved before attempting another removal.
