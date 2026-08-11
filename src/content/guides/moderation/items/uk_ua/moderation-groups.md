---
Модератори можуть бути розміщені в групи для модерації різних сторінок або категорій контенту.

Коли модератор належить одній або декільком групам, він бачитиме лише коментарі з цих груп на сторінці «Модерація коментарів».

Наприклад, уявімо, що ми керуємо сайтом, який показує відео за категоріями. Ми можемо захотіти мати різних модераторів для відео про котів, собак і папуг, тому [додамо ці групи](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups).

[app-screenshot-start url='/auth/my-account/moderate-comments/moderation-groups?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderation-groups'; selector = '.content'; alt='Список груп модерації з групами Cat, Dog і Parrot, створеними для кожної категорії відео'; title='Сторінка груп модерації' app-screenshot-end]

Коли ми додаємо модератора, у нас з'являється можливість вибрати одну або декілька груп, до яких буде належати модератор:

[app-screenshot-start url='/auth/my-account/moderate-comments/moderator/new?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderator/new'; selector = '.account-block'; alt='Форма додавання модератора з селектором груп, який використовується для призначення модератора до однієї або декількох груп'; title='Додавання модератора та вибір групи' app-screenshot-end]

Нарешті, коментарі потрібно прив’язати до однієї або декількох груп, щоб правильні модератори їх бачили.

Це можна налаштувати, [додавши деякі групи](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) і вказавши відповідні ідентифікатори `Moderation Group` у віджеті коментарів,
[як зазначено тут](/guide-customizations-and-configuration.html#moderation-group-ids).