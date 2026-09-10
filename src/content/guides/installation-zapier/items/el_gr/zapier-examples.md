## Example Zaps

A few workflows that take minutes to set up.

**Λάβετε ειδοποίηση για νέα σχόλια.** New Comment, then Slack "Send Channel Message" or Discord "Send
Channel Message". Map the commenter name, the comment text, and the page URL into the message. Add the
domain filter to notify a different channel per site.

**Διατηρήστε ένα αρχείο για κάθε σχόλιο.** New Comment, then Google Sheets "Create Spreadsheet Row". Add Deleted
Comment as a second Zap that appends a row with the comment id, so the sheet doubles as an audit trail.

**Στείλτε email στον συγγραφέα όταν ένα σχόλιο εγκριθεί.** Updated Comment with a Zapier filter on Approved is true,
then Gmail "Send Email". Because Updated Comment fires on every change, the filter is what makes this Zap
react only to approvals.

**Προσθέστε τους σχολιαστές στο CRM ή στη λίστα αλληλογραφίας σας.** New Comment, then HubSpot "Create or Update Contact" or
Mailchimp "Add or Update Subscriber" using the commenter email. Respect your privacy policy and local law
before adding anyone to a marketing list.

**Δημιουργήστε ένα σχόλιο από μια φόρμα.** Typeform or Google Forms "New Response", then FastComments Create Comment
with the page URL ID your site uses for testimonials. Leave Approved unchecked to review each one before it
appears.

**Δημοσιεύστε ανακοινώσεις σε ένα feed.** RSS by Zapier "New Item in Feed", then Create Feed Post with the item's
title, content, and link.

**Παρέχετε μέλη ως χρήστες SSO.** Memberstack, Memberful, or your own webhook, then Find SSO User followed
by Create SSO User in "find or create" mode.

**Αναβάθμιση αναφερόμενων σχολίων.** Updated Comment, filtered on a flag count above zero, then Trello "Create
Card" or Linear "Create Issue" with the comment id and a link to the moderation page.

**Δημοσιεύστε σελίδες καθώς γίνονται ζωντανές.** WordPress or Ghost "New Post", then Create Page with the post URL, so the
page is listed and restricted before the first comment.

**Αρχειοθετήστε διαγραμμένα σχόλια.** Deleted Comment, then Airtable "Create Record" with the full comment for
compliance retention.