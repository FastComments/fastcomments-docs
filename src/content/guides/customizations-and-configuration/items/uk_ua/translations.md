[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

За допомогою FastComments весь текст у віджеті коментарів можна налаштувати.

Ви можете замінити окремий фрагмент тексту, наприклад кнопку надсилання, або весь текст у всьому віджеті коментарів.

За замовчуванням текст у віджеті коментарів перекладається відповідно до локалі користувача. Однак ми можемо замінити текст, якщо впевнені, що наша база користувачів використовує одну й ту ж локаль/мову, наприклад:

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'Користувацький текст'; code-example-end]

Усі налаштовувані переклади можна знайти <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">тут</a> під вкладкою "розширені параметри".

Однак існує простіший спосіб — через інтерфейс налаштування віджета. Там ми можемо просто знайти текст, який відображається у віджеті коментування в локалі EN_US, і вказати його заміну.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; alt='Панель користувацького тексту з рядком віджета, вибраним у випадаючому списку, та полем заміни тексту'; title='Користувацький текст' app-screenshot-end]

Всі заміни перекладів наразі впливають на всі локалі.

---