Щоб плагін працював, token зберігається в базі даних вашого WordPress і також у вашому обліковому записі FastComments. Коли плагін надсилає запит на наші сервери, він надає
цей token.

Ви можете переглянути всі інтеграції, авторизовані у вашому обліковому записі FastComments, [тут](https://fastcomments.com/auth/my-account/manage-data/integrations).

Вся комунікація відбувається через HTTPS.

Вся комунікація є *вихідною* з вашого сервера WordPress *до* FastComments.com, включаючи синхронізацію *назад* у вашу інсталяцію WordPress, оскільки вона реалізується через [polling](https://en.wikipedia.org/wiki/Polling_(computer_science)) із налаштування [cron](https://developer.wordpress.org/plugins/cron/) у вашій інсталяції WordPress.