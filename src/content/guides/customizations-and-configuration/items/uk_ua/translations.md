[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

У FastComments увесь текст у віджеті коментарів налаштовується.

Ви можете замінити окремий фрагмент тексту, наприклад кнопку відправки, або весь текст у віджеті коментарів.

За замовчуванням, текст у віджеті коментарів перекладається залежно від локалі користувача. Однак ми можемо перевизначити текст, якщо ми впевнені
що наша база користувачів використовує ту саму локаль/мову, наприклад:

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'Custom Text'; code-example-end]

Усі налаштовувані переклади можна знайти <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">тут</a> на вкладці "додаткові параметри".

Проте є простіший спосіб — через інтерфейс налаштування віджета. Там ми можемо просто знайти текст, який показується у віджеті коментарів для локалі EN_US, і вказати
заміну.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; title='Custom Text' app-screenshot-end]

Усі перевизначення перекладів наразі впливають на всі локалі.