По подразумевању, корисници могу да обришу своје коментаре. Такође, брисањем свог коментара аутоматски
бришу се сви подређени и привремени коментари у нити. Ово понашање је такође активно.

You can restrict this in the following ways:

- Уместо тога, анонимизујте обрисани коментар (подесите име и текст на `[deleted]` или прилагођену вредност).
- Не дозвољавајте брисање коментара када постоје одговори. Приказује се прилагодљива порука о грешци.
- Ограничите брисање коментара који имају одговоре само на администраторе и модераторе.

This can be configured via the `Comment Thread Deletion` section in the Widget Customization UI.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.comment-thread-deletion-mode']; selector = '.comment-thread-deletion-mode'; title='Customize Delete Behavior for Replies' app-screenshot-end]

---