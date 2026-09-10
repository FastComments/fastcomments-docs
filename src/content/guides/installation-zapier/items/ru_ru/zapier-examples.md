## Примеры Zaps

Несколько рабочих процессов, которые настраиваются за считанные минуты.

**Получать уведомления о новых комментариях.** New Comment, then Slack "Send Channel Message" or Discord "Send
Channel Message". Сопоставьте имя комментатора, текст комментария и URL страницы в сообщение. Добавьте
фильтр домена, чтобы уведомлять разные каналы для каждого сайта.

**Вести журнал каждого комментария.** New Comment, then Google Sheets "Create Spreadsheet Row". Добавьте Deleted
Comment в качестве второго Zap, который добавляет строку с идентификатором комментария, так что лист служит аудиторским журналом.

**Отправлять электронное письмо автору, когда комментарий одобрен.** Updated Comment с фильтром Zapier, где Approved = true,
then Gmail "Send Email". Поскольку Updated Comment срабатывает при каждом изменении, фильтр обеспечивает, что этот Zap
реагирует только на одобрения.

**Добавлять комментаторов в вашу CRM или список рассылки.** New Comment, then HubSpot "Create or Update Contact" or
Mailchimp "Add or Update Subscriber" using the commenter email. Соблюдайте вашу политику конфиденциальности и местное законодательство,
прежде чем добавлять кого-либо в маркетинговый список.

**Создавать комментарий из формы.** Typeform or Google Forms "New Response", then FastComments Create Comment
with the page URL ID your site uses for testimonials. Оставьте Approved без отметки, чтобы проверять каждый комментарий перед его
появлением.

**Публиковать объявления в ленту.** RSS by Zapier "New Item in Feed", then Create Feed Post with the item's
title, content, and link.

**Предоставлять членам доступ как SSO пользователям.** Memberstack, Memberful, or your own webhook, then Find SSO User followed
by Create SSO User in "find or create" mode.

**Эскалировать отмеченные комментарии.** Updated Comment, filtered on a flag count above zero, then Trello "Create
Card" or Linear "Create Issue" with the comment id and a link to the moderation page.

**Публиковать страницы по их публикации.** WordPress or Ghost "New Post", then Create Page with the post URL, so the
page is listed and restricted before the first comment.

**Архивировать удалённые комментарии.** Deleted Comment, then Airtable "Create Record" with the full comment for
compliance retention.