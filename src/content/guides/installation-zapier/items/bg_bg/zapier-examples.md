## Примери за Zaps

Няколко работни процеса, които се настройват за минути.

**Получавайте известия за нови коментари.** New Comment, then Slack "Send Channel Message" or Discord "Send Channel Message". Картографирайте името на коментатора, текста на коментара и URL‑то на страницата в съобщението. Добавете филтър за домейн, за да известявате различен канал за всеки сайт.

**Водете регистър на всеки коментар.** New Comment, then Google Sheets "Create Spreadsheet Row". Добавете Deleted Comment като втори Zap, който добавя ред с идентификатора на коментара, така че листът да служи и като одитен журнал.

**Изпратете имейл до автора, когато коментарът бъде одобрен.** Updated Comment с филтър в Zapier за Approved е истина, then Gmail "Send Email". Тъй като Updated Comment се задейства при всяка промяна, филтърът е това, което кара този Zap да реагира само на одобрения.

**Добавете коментаторите към вашата CRM или имейл листа.** New Comment, then HubSpot "Create or Update Contact" or Mailchimp "Add or Update Subscriber", използвайки имейла на коментатора. Спазвайте вашата политика за поверителност и местното законодателство, преди да добавяте някого към маркетинг листа.

**Създайте коментар от форма.** Typeform or Google Forms "New Response", then FastComments Create Comment с ID‑то на URL‑то на страницата, което вашият сайт използва за отзиви. Оставете Approved без отметка, за да преглеждате всеки коментар преди да се публикува.

**Публикувайте обявления във фийд.** RSS by Zapier "New Item in Feed", then Create Feed Post с заглавието, съдържанието и връзката на елемента.

**Осигурете членове като SSO потребители.** Memberstack, Memberful, or your own webhook, then Find SSO User followed by Create SSO User in "find or create" mode.

**Ескалирайте докладвани коментари.** Updated Comment, filtered on a flag count above zero, then Trello "Create Card" or Linear "Create Issue" с идентификатора на коментара и връзка към страницата за модериране.

**Публикувайте страници, когато станат активни.** WordPress or Ghost "New Post", then Create Page с URL‑то на публикацията, така че страницата да бъде листната и ограничена преди първия коментар.

**Архивирайте изтритите коментари.** Deleted Comment, then Airtable "Create Record" с пълния коментар за съхранение съгласно изискванията за запазване.