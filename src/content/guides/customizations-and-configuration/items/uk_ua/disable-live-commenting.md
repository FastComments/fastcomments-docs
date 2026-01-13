[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

За замовчуванням у FastComments живе коментування увімкнено.

Це означає, що всі глядачі треду коментарів бачать однаковий вміст.

Наприклад, якщо додається коментар, цей коментар має відобразитися. Якщо коментар редагується або видаляється,
то ці коментарі також будуть редаговані або видалені для всіх глядачів треду. Те саме стосується голосів і всіх дій модерації.

Однак це можна вимкнути:

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Вимкнути живе коментування'; code-example-end]

Це також можна зробити без коду. На сторінці налаштування віджета див. розділ "Вимкнути живе коментування".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; title='Вимкнути живе коментування' app-screenshot-end]

---