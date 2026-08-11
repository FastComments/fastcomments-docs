[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

За замовчуванням FastComments матиме ввімкнене живе коментування.

Це означає, що кожен глядач потоку коментарів має бачити один і той самий вміст.

Наприклад, якщо додати коментар, цей коментар має з'явитися. Якщо коментар відредаговано або видалено,
то ці коментарі будуть відредаговані або видалені для всіх глядачів потоку. Те ж саме щодо голосувань та всіх дій модерації.

Проте ми можемо це вимкнути:

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Вимкнути живе коментування'; code-example-end]

Це також можна зробити без коду. На сторінці налаштування віджету перегляньте розділ "Вимкнути живе коментування".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; alt='Розділ \"Вимкнути живе коментування\" на сторінці налаштування віджету, вимикає оновлення потоку в реальному часі'; title='Вимкнути живе коментування' app-screenshot-end]

---