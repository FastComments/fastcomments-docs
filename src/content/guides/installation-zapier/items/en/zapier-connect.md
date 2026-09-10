## Connect Your Account

1. In Zapier, add a FastComments step to a Zap, or open the FastComments app page in the Zapier App Directory.
2. Choose **Sign in to FastComments**. Zapier asks for your region first: pick **United States** unless your
   account was created on the EU region (`eu.fastcomments.com`).
3. A FastComments window opens. Sign in if you are not already signed in.
4. Review the consent page. It shows the Zapier application, the account it will be connected to, and the
   permissions requested (read and write). Choose **Approve**.
5. Zapier stores the connection and labels it with your site name and username.

The connection uses OAuth. No API key is copied into Zapier, and the token Zapier holds only works for the
account you approved.

## Who can connect

The person approving the connection must be an **API admin** on the FastComments account. Account owners
have this permission; other team members can be granted it on the Users page. Someone without it sees a
"you do not have permission" page instead of the consent form.

## Connecting the right site

The consent page connects the account you are currently signed in to. If you manage several accounts,
switch to the right one from the account switcher before approving, or use the **switch account** link on
the consent page. The connection label in Zapier shows the site name, so a wrong choice is easy to spot.

## Reviewing and revoking access

Every connection appears under **Connected Apps** in the FastComments dashboard, with the permissions it
holds and when it was last used. Revoking it there disconnects Zapier immediately; any Zap using that
connection stops until it is reconnected. You can also remove the connection from the Zapier side under
**My Apps**.
