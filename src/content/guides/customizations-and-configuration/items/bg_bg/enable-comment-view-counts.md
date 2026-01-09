[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

По подразбиране FastComments не следи кой е прегледал всеки коментар и не предоставя статистика за това.

Въпреки това, можем да активираме тази функция и системата ще започне да следи, когато всеки потребител превърти до коментар.

Когато това се случи, броячът до икона с око, показан на всеки коментар, ще бъде увеличен. Броячът се обновява в реално време и се съкращава според локала на потребителя.

Можем да активираме това като зададем флага **enableViewCounts** на true:

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Enabling Comment View Counts'; code-example-end]

Това може да се персонализира без код, на страницата за персонализиране на уиджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; title='Enabling Comment View Counts' app-screenshot-end]

Ние следим идентификатора на потребителя*, който е прегледал коментара, така че ако прегледате коментара отново, той да не се увеличава. Ако прегледате коментара отново
след две години, броячът ще се увеличи отново.

- *Забележка: или anon session id, или потребителския IP като хеширана стойност.