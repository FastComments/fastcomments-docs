По подразумеваној поставци, корисници могу обрисати своје коментаре. Такође, брисање њиховог коментара аутоматски
брише све подређене и привремене коментаре у нити. Ово понашање је такође активно у реалном времену.

You can restrict this in the following ways:

- Уместо тога, анонимизујте обрисани коментар (поставите име и текст на `[deleted]` или прилагођену вредност).
- Не дозвољавајте брисање коментара када постоје одговори. Приказује се прилагодљива порука о грешци.
- Ограничите могућност брисања коментара који имају одговоре само на администраторе и модераторе.

This can be configured via the `Comment Thread Deletion` section in the Widget Customization UI.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.comment-thread-deletion-mode']; selector = '.comment-thread-deletion-mode'; title='Customize Delete Behavior for Replies' app-screenshot-end]

---