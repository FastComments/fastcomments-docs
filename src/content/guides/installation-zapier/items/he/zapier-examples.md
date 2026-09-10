## דוגמאות ל‑Zaps

כמה זרימות עבודה שלוקחות דקות להקמה.

**קבלו התראה על תגובות חדשות.** New Comment, then Slack "Send Channel Message" or Discord "Send Channel Message". Map the commenter name, the comment text, and the page URL into the message. Add the domain filter to notify a different channel per site.

**שמרו יומן של כל תגובה.** New Comment, then Google Sheets "Create Spreadsheet Row". Add Deleted Comment as a second Zap that appends a row with the comment id, so the sheet doubles as an audit trail.

**שלחו אימייל למחבר כאשר תגובה מאושרת.** Updated Comment with a Zapier filter on Approved is true, then Gmail "Send Email". Because Updated Comment fires on every change, the filter is what makes this Zap react only to approvals.

**הוסיפו מגיבים למערכת ה‑CRM או לרשימת התפוצה שלכם.** New Comment, then HubSpot "Create or Update Contact" or Mailchimp "Add or Update Subscriber" using the commenter email. Respect your privacy policy and local law before adding anyone to a marketing list.

**צרו תגובה מטופס.** Typeform or Google Forms "New Response", then FastComments Create Comment with the page URL ID your site uses for testimonials. Leave Approved unchecked to review each one before it appears.

**פרסמו הודעות לפיד.** RSS by Zapier "New Item in Feed", then Create Feed Post with the item's title, content, and link.

**הקצו חברים כמשתמשי SSO.** Memberstack, Memberful, or your own webhook, then Find SSO User followed by Create SSO User in "find or create" mode.

**העלו תגובות מדווחות.** Updated Comment, filtered on a flag count above zero, then Trello "Create Card" or Linear "Create Issue" with the comment id and a link to the moderation page.

**פרסמו דפים כאשר הם מתפרסמים.** WordPress or Ghost "New Post", then Create Page with the post URL, so the page is listed and restricted before the first comment.

**ארכיבו תגובות שנמחקו.** Deleted Comment, then Airtable "Create Record" with the full comment for compliance retention.