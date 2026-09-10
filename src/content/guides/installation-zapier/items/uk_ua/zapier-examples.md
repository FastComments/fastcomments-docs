---
## Приклади Zap'ів

Кілька робочих процесів, які займають кілька хвилин для налаштування.

**Отримуйте сповіщення про нові коментарі.** New Comment, then Slack "Send Channel Message" or Discord "Send Channel Message". Відобразіть ім'я коментатора, текст коментаря та URL сторінки у повідомленні. Додайте фільтр домену, щоб сповіщати різний канал для кожного сайту.

**Ведіть журнал кожного коментаря.** New Comment, then Google Sheets "Create Spreadsheet Row". Додайте Deleted Comment як другий Zap, який додає рядок з ідентифікатором коментаря, тож лист слугує аудиторським журналом.

**Надсилайте електронний лист автору, коли коментар схвалено.** Updated Comment with a Zapier filter on Approved is true, then Gmail "Send Email". Оскільки Updated Comment спрацьовує при кожній зміні, фільтр забезпечує, що цей Zap реагує лише на схвалення.

**Додавайте коментаторів у вашу CRM або список розсилки.** New Comment, then HubSpot "Create or Update Contact" or Mailchimp "Add or Update Subscriber" using the commenter email. Дотримуйтесь вашої політики конфіденційності та місцевого законодавства перед додаванням когось до маркетингового списку.

**Створюйте коментар з форми.** Typeform or Google Forms "New Response", then FastComments Create Comment with the page URL ID your site uses for testimonials. Залиште Approved незакресленим, щоб переглянути кожен коментар перед його появою.

**Публікуйте оголошення у стрічці.** RSS by Zapier "New Item in Feed", then Create Feed Post with the item's title, content, and link.

**Надавайте членам доступ як користувачам SSO.** Memberstack, Memberful, or your own webhook, then Find SSO User followed by Create SSO User in "find or create" mode.

**Ескалюйте повідомлені коментарі.** Updated Comment, filtered on a flag count above zero, then Trello "Create Card" or Linear "Create Issue" with the comment id and a link to the moderation page.

**Публікуйте сторінки, коли вони стають активними.** WordPress or Ghost "New Post", then Create Page with the post URL, so the page is listed and restricted before the first comment.

**Архівуйте видалені коментарі.** Deleted Comment, then Airtable "Create Record" with the full comment for compliance retention.

---