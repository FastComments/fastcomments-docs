[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

За замовчуванням FastComments не відстежує, хто переглянув кожен коментар, і не надає жодної статистики з цього приводу.

Однак цю функцію можна увімкнути, і тоді система почне відстежувати перегляди під час того, як користувачі прокручують до коментаря.

Коли це відбувається, лічильник поруч із значком ока на кожному коментарі збільшуватиметься. Лічильник оновлюється в реальному часі та скорочується відповідно до локалі користувача.

Ми можемо увімкнути це, встановивши прапорець **enableViewCounts** у true:

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Enabling Comment View Counts'; code-example-end]

Це можна налаштувати без коду на сторінці налаштування віджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; title='Enabling Comment View Counts' app-screenshot-end]

Ми відстежуємо id користувача*, який переглянув коментар, тож якщо ви переглянете коментар ще раз, лічильник не збільшиться. Якщо ви переглянете коментар через два роки, лічильник знову збільшиться.

- *Примітка: або анонімний id сесії, або IP користувача у вигляді хешованого значення.

---