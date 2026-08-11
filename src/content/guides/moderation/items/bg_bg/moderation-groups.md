---
Модераторите могат да бъдат поставени в групи, за да модерират различни страници или категории съдържание.

Когато модератор принадлежи към една или повече групи, той ще вижда само коментари от тези групи в страницата „Модериране на коментари“.

Например, да кажем, че управляваме сайт, който показва видеа по категории. Може да искаме различни модератори за видеа с котки, кучета и папагали, затова [да добавим тези групи](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups).

[app-screenshot-start url='/auth/my-account/moderate-comments/moderation-groups?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderation-groups'; selector = '.content'; alt='Списък с модераторски групи, включващ групите Котка, Куче и Папагал, създадени за всяка видео категория'; title='Страницата с модераторски групи' app-screenshot-end]

Когато добавим модератор, сега имаме възможност да изберем една или повече групи, към които модераторът ще принадлежи:

[app-screenshot-start url='/auth/my-account/moderate-comments/moderator/new?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderator/new'; selector = '.account-block'; alt='Формуляр за добавяне на модератор с избирач на група, използван за присвояване на модератора към една или повече групи'; title='Добавяне на модератор и избор на група' app-screenshot-end]

Накрая, коментарите трябва да бъдат свързани с една или повече групи, за да ги видят правилните модератори.

Това може да се настрои, като [добавите някои групи](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) и след това посочите съответните `Moderation Group` идентификатори в уиджета за коментари,
[както е указано тук](/guide-customizations-and-configuration.html#moderation-group-ids).

---