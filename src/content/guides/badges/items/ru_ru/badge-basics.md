FastComments User Badges are configured by administrators with the `Customize Data` permission.

This is done via [Customize -> Badges](https://fastcomments.com/auth/my-account/configure-badges) in your admin dashboard.

When a user is awarded a badge it becomes displayed on their profile and on their comments.

When adding a badge we can setup a `Display Label`, which is the name the user sees associated with the badge. For example, if we add a `Comment Count` badge
we probably don't want to show that technical name because it's very lame. We might call it `Super Member` or similar. Badges can also stack and replace each other, as we'll cover
later in this document.

Badges also have configurable thresholds.

Badges can be created, and then later disabled by unchecking `Enabled`. Disabling a badge means it will no longer be automatically awarded, and won't show in the Award Manual Badge menu, but
users will keep the badge.

### Badge Display Types

Badges can be images or text badges, which support some basic styling (text color, background color, and border color). You can also style badges via CSS.

Image badges can be GIF images to show animated badges.

### Tip - Do Not Remove Badges!

Users love badges. They tend to really care about them, even if it's a bug that you added by mistake, and you want to change the badge icon.

If we've learned anything, it's extremely difficult to take something away from users. Removing a badge because you as an owner of the site no
longer like it, or want to make changes, may result in a very angry crowd of users that suddenly leave your site out of frustration. For this reason
`Delete` was not even an option for the first few months we released this feature - however we ended up having to add it. But please, use delete with caution. We have
seen many long time, multi-year, users get very frustrated and leave their communities because administrators decided to delete a badge.

If you must stop using a badge suggest you simply disable it so that users keep their badge.

### Badge Reprocessing

When a badge is added or changed, the system will retroactively check anyone that interacted with your site to see if they should get the badge. This will be
visible in the Badges page in the admin dashboard, as a spinner will be shown instead of the number of users that have the badge. This is because the number of users
is being determined.

### Seeing Who Has a Badge

In the Badges list each link has a `View Users` option to show the list of users which has earned or was manually awarded a badge.