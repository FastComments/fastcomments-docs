При модерировании и просмотре потоков комментариев желательно иметь возможность переходить непосредственно к нужной ветке, чтобы получить контекст во время модерации.

This means that the user's flow starts in the Comment Moderation page, and would then have to go from an individual comment to
the page containing that comment, wait for that page to load, wait for the comments to load, and then scroll to that comment.

Однако FastComments предлагает более быстрый способ. На странице "Модерирование комментариев", рядом с каждым комментарием, в правом нижнем углу находится кнопка "Просмотреть комментарий".

[app-screenshot-start url='/auth/my-account/moderate-comments?filter=&text-search=&page=1&count=1&demo=true'; linkUrl='/auth/my-account/moderate-comments'; selector = '.comments .comment-component'; title='A Comment' app-screenshot-end]

Если у этого комментария есть ответы, на кнопке вместо этого будет указано количество ответов, но при нажатии действие будет то же самое.

Эта кнопка перенаправит вас в **Просмотр потока комментариев**.

The Comment Thread Viewer is a small, fast loading application hosted by FastComments that renders the comment thread for the page that
the comment is on, and scrolls to that comment.

Это позволяет модераторам быстро получить необходимый контекст, не дожидаясь загрузки другой страницы.

---