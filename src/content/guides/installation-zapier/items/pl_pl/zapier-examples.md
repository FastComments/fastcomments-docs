## Example Zaps

A few workflows that take minutes to set up.

**Otrzymuj powiadomienia o nowych komentarzach.** New Comment, then Slack "Send Channel Message" or Discord "Send
Channel Message". Map the commenter name, the comment text, and the page URL into the message. Add the
domain filter to notify a different channel per site.

**Prowadź dziennik każdego komentarza.** New Comment, then Google Sheets "Create Spreadsheet Row". Add Deleted
Comment as a second Zap that appends a row with the comment id, so the sheet doubles as an audit trail.

**Wyślij e‑mail do autora, gdy komentarz zostanie zatwierdzony.** Updated Comment with a Zapier filter on Approved is true,
then Gmail "Send Email". Because Updated Comment fires on every change, the filter is what makes this Zap
react only to approvals.

**Dodaj komentujących do swojego CRM lub listy mailingowej.** New Comment, then HubSpot "Create or Update Contact" or
Mailchimp "Add or Update Subscriber" using the commenter email. Respect your privacy policy and local law
before adding anyone to a marketing list.

**Utwórz komentarz z formularza.** Typeform or Google Forms "New Response", then FastComments Create Comment
with the page URL ID your site uses for testimonials. Leave Approved unchecked to review each one before it
appears.

**Publikuj ogłoszenia w kanale.** RSS by Zapier "New Item in Feed", then Create Feed Post with the item's
title, content, and link.

**Provisionuj członków jako użytkowników SSO.** Memberstack, Memberful, or your own webhook, then Find SSO User followed
by Create SSO User in "find or create" mode.

**Zgłaszaj eskalowane komentarze.** Updated Comment, filtered on a flag count above zero, then Trello "Create
Card" or Linear "Create Issue" with the comment id and a link to the moderation page.

**Publikuj strony w momencie ich uruchomienia.** WordPress or Ghost "New Post", then Create Page with the post URL, so the
page is listed and restricted before the first comment.

**Archiwizuj usunięte komentarze.** Deleted Comment, then Airtable "Create Record" with the full comment for
compliance retention.