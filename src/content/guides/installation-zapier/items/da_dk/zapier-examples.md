## Eksempel Zaps

Et par arbejdsgange, der tager få minutter at sætte op.

**Få besked om nye kommentarer.** New Comment, then Slack "Send Channel Message" or Discord "Send Channel Message". Map the commenter name, the comment text, and the page URL into the message. Add the domain filter to notify a different channel per site.

**Hold en log over hver kommentar.** New Comment, then Google Sheets "Create Spreadsheet Row". Add Deleted Comment as a second Zap that appends a row with the comment id, so the sheet doubles as an audit trail.

**E‑mail forfatteren, når en kommentar er godkendt.** Updated Comment with a Zapier filter on Approved is true, then Gmail "Send Email". Because Updated Comment fires on every change, the filter is what makes this Zap react only to approvals.

**Tilføj kommentatorer til dit CRM eller mailingliste.** New Comment, then HubSpot "Create or Update Contact" or Mailchimp "Add or Update Subscriber" using the commenter email. Respect your privacy policy and local law before adding anyone to a marketing list.

**Opret en kommentar fra en formular.** Typeform or Google Forms "New Response", then FastComments Create Comment with the page URL ID your site uses for testimonials. Leave Approved unchecked to review each one before it appears.

**Post meddelelser til et feed.** RSS by Zapier "New Item in Feed", then Create Feed Post with the item's title, content, and link.

**Provisioner medlemmer som SSO‑brugere.** Memberstack, Memberful, or your own webhook, then Find SSO User followed by Create SSO User in "find or create" mode.

**Eskaler rapporterede kommentarer.** Updated Comment, filtered on a flag count above zero, then Trello "Create Card" or Linear "Create Issue" with the comment id and a link to the moderation page.

**Publicer sider, når de går live.** WordPress or Ghost "New Post", then Create Page with the post URL, so the page is listed and restricted before the first comment.

**Arkiver slettede kommentarer.** Deleted Comment, then Airtable "Create Record" with the full comment for compliance retention.